//! Embed the original brand icon and Windows product identity without a build dependency.
use std::{env, fs, path::PathBuf, process::Command};
fn word(b: &mut Vec<u8>, v: u16) {
    b.extend(v.to_le_bytes());
}
fn dword(b: &mut Vec<u8>, v: u32) {
    b.extend(v.to_le_bytes());
}
fn align(b: &mut Vec<u8>) {
    while b.len() % 4 != 0 {
        b.push(0);
    }
}
fn utf(s: &str) -> Vec<u8> {
    s.encode_utf16()
        .chain(Some(0))
        .flat_map(u16::to_le_bytes)
        .collect()
}
fn resource(out: &mut Vec<u8>, ty: u16, id: u16, data: &[u8]) {
    dword(out, data.len() as u32);
    dword(out, 32);
    word(out, 0xffff);
    word(out, ty);
    word(out, 0xffff);
    word(out, id);
    dword(out, 0);
    word(out, 0x1030);
    word(out, 0x409);
    dword(out, 0);
    dword(out, 0);
    out.extend(data);
    align(out);
}
fn block(key: &str, kind: u16, value: &[u8], children: &[Vec<u8>]) -> Vec<u8> {
    let mut b = Vec::new();
    word(&mut b, 0);
    word(
        &mut b,
        if kind == 1 {
            (value.len() / 2) as u16
        } else {
            value.len() as u16
        },
    );
    word(&mut b, kind);
    b.extend(utf(key));
    align(&mut b);
    b.extend(value);
    for child in children {
        align(&mut b);
        b.extend(child);
    }
    let n = b.len() as u16;
    b[0..2].copy_from_slice(&n.to_le_bytes());
    b
}
fn main() {
    println!("cargo:rerun-if-changed=assets/rs7.ico");
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }
    let ico = fs::read("assets/rs7.ico").expect("RS7 icon");
    let count = u16::from_le_bytes([ico[4], ico[5]]) as usize;
    let mut res = Vec::new();
    resource(&mut res, 0, 0, &[]);
    let mut group = vec![0, 0, 1, 0];
    word(&mut group, count as u16);
    for n in 0..count {
        let e = &ico[6 + n * 16..6 + (n + 1) * 16];
        let size = u32::from_le_bytes(e[8..12].try_into().unwrap()) as usize;
        let offset = u32::from_le_bytes(e[12..16].try_into().unwrap()) as usize;
        resource(&mut res, 3, n as u16 + 1, &ico[offset..offset + size]);
        group.extend(&e[..12]);
        word(&mut group, n as u16 + 1);
    }
    resource(&mut res, 14, 1, &group);
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let major = env::var("CARGO_PKG_VERSION_MAJOR")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let patch = env::var("CARGO_PKG_VERSION_PATCH")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut fixed = Vec::new();
    for v in [
        0xFEEF04BD,
        0x10000,
        (major << 16) | minor,
        patch << 16,
        (major << 16) | minor,
        patch << 16,
        0x3f,
        if env::var("CARGO_PKG_VERSION_PRE")
            .unwrap_or_default()
            .is_empty()
        {
            0
        } else {
            2
        },
        0x40004,
        1,
        0,
        0,
        0,
    ] {
        dword(&mut fixed, v);
    }
    let strings = [
        ("FileDescription", "RS7"),
        ("ProductName", "RS7"),
        ("InternalName", "RS7"),
        ("OriginalFilename", "RS7.exe"),
        ("FileVersion", version.as_str()),
        ("ProductVersion", version.as_str()),
    ]
    .iter()
    .map(|(k, v)| block(k, 1, &utf(v), &[]))
    .collect::<Vec<_>>();
    let table = block("040904b0", 1, &[], &strings);
    let string_info = block("StringFileInfo", 1, &[], &[table]);
    let translation = block("Translation", 0, &[9, 4, 176, 4], &[]);
    let vars = block("VarFileInfo", 1, &[], &[translation]);
    let info = block("VS_VERSION_INFO", 0, &fixed, &[string_info, vars]);
    resource(&mut res, 16, 1, &info);
    let manifest=br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0"><assemblyIdentity version="1.1.0.0" name="RS7" type="win32"/><description>RS7</description><dependency><dependentAssembly><assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*"/></dependentAssembly></dependency><trustInfo xmlns="urn:schemas-microsoft-com:asm.v3"><security><requestedPrivileges><requestedExecutionLevel level="asInvoker" uiAccess="false"/></requestedPrivileges></security></trustInfo></assembly>"#;
    resource(&mut res, 24, 1, manifest);
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let file = out.join("rs7.res");
    fs::write(&file, res).unwrap();
    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        println!("cargo:rustc-link-arg={}", file.display());
    } else {
        let object = out.join("rs7-resource.o");
        let target = if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
            "pe-x86-64"
        } else {
            "pe-i386"
        };
        let status = Command::new(env::var("WINDRES").unwrap_or_else(|_| "windres".into()))
            .arg("-J")
            .arg("res")
            .arg("-O")
            .arg("coff")
            .arg("-F")
            .arg(target)
            .arg(&file)
            .arg(&object)
            .status()
            .expect("GNU windres is required for Windows icon resources");
        assert!(status.success(), "RS7 resource conversion failed");
        println!("cargo:rustc-link-arg={}", object.display());
    }
}
