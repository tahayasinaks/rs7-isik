using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.InteropServices;
public static class LightingKeys {
  public static void Main(string[] args) { Run(int.Parse(args[0])); }
  private delegate IntPtr HookProc(int code, IntPtr w, IntPtr l);
  private static HookProc callback = Handle;
  private static IntPtr hook;
  private static HashSet<int> held = new HashSet<int>();
  [StructLayout(LayoutKind.Sequential)] private struct KeyInfo { public uint vk, scan, flags, time; public UIntPtr extra; }
  [StructLayout(LayoutKind.Sequential)] private struct Msg { public IntPtr hwnd; public uint message; public UIntPtr wParam; public IntPtr lParam; public uint time; public int x, y; public uint priv; }
  [DllImport("user32.dll", SetLastError=true)] private static extern IntPtr SetWindowsHookEx(int id, HookProc proc, IntPtr module, uint thread);
  [DllImport("user32.dll")] private static extern bool UnhookWindowsHookEx(IntPtr handle);
  [DllImport("user32.dll")] private static extern IntPtr CallNextHookEx(IntPtr handle, int code, IntPtr w, IntPtr l);
  [DllImport("user32.dll")] private static extern int GetMessage(out Msg msg, IntPtr hwnd, uint min, uint max);
  [DllImport("kernel32.dll", CharSet=CharSet.Unicode)] private static extern IntPtr GetModuleHandle(string name);
  private static IntPtr Handle(int code, IntPtr w, IntPtr l) {
    if (code >= 0) {
      KeyInfo key = (KeyInfo)Marshal.PtrToStructure(l, typeof(KeyInfo));
      int scan = (int)key.scan + ((key.flags & 1) != 0 ? 256 : 0);
      int message = w.ToInt32();
      if ((key.flags & 16) == 0) {
        if (message == 256 || message == 260) { if (held.Add(scan)) Console.WriteLine(scan); }
        if (message == 257 || message == 261) held.Remove(scan);
      }
    }
    return CallNextHookEx(hook, code, w, l);
  }
  public static void Run(int owner) {
    var timer = new System.Threading.Timer(_ => { try { if (Process.GetProcessById(owner).HasExited) Environment.Exit(0); } catch { Environment.Exit(0); } }, null, 1000, 1000);
    hook = SetWindowsHookEx(13, callback, GetModuleHandle(null), 0);
    if (hook == IntPtr.Zero) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
    Console.WriteLine("READY");
    try { Msg msg; while (GetMessage(out msg, IntPtr.Zero, 0, 0) > 0) {} }
    finally { UnhookWindowsHookEx(hook); timer.Dispose(); }
  }
}