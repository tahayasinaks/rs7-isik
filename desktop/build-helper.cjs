const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const output = path.join(__dirname, 'bin');
fs.mkdirSync(output, { recursive: true });
const compiler = path.join(process.env.SystemRoot || 'C:\\Windows', 'Microsoft.NET', 'Framework64', 'v4.0.30319', 'csc.exe');
execFileSync(compiler, ['/nologo', '/target:exe', '/optimize+', '/out:' + path.join(output,'RS7-Keys.exe'), path.join(__dirname,'keyboard-helper.cs')], { stdio: 'inherit', windowsHide: true });
