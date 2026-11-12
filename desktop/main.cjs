const { app, BrowserWindow, ipcMain, session } = require('electron');
const { spawn } = require('node:child_process');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
let win, helper, allowClose = false, closeTimer;
const ids = [4640,4641,4651,4695,4696,4921,4917,4888];
const page = pathToFileURL(path.join(__dirname, '..', 'index.html')).href;
function stopInput() { const child = helper; helper = null; child?.kill(); }
function startInput() {
  if (helper) return Promise.resolve();
  return new Promise((resolve, reject) => {
    const binary = app.isPackaged ? path.join(process.resourcesPath, 'RS7-Keys.exe') : path.join(__dirname, 'bin', 'RS7-Keys.exe');
    const child = spawn(binary, [String(process.pid)], { windowsHide: true, stdio: ['ignore','pipe','pipe'] });
    helper = child;
    let ready = false, buffer = '';
    const timer = setTimeout(() => { if (helper === child) stopInput(); reject(new Error('Tuş dinleyicisi başlatılamadı.')); }, 15000);
    child.stdout.on('data', data => {
      buffer += data.toString(); const lines = buffer.split(/\r?\n/); buffer = lines.pop();
      for (const line of lines) {
        if (line === 'READY') { ready = true; clearTimeout(timer); resolve(); }
        else if (/^\d{1,3}$/.test(line) && helper === child && win && !win.isDestroyed()) win.webContents.send('lighting-key', Number(line));
      }
    });
    child.stderr.on('data', () => {});
    child.on('error', () => { clearTimeout(timer); if (helper === child) helper = null; reject(new Error('Windows tuş dinleyicisi açılamadı.')); });
    child.on('exit', () => { clearTimeout(timer); if (helper === child) { helper = null; if (ready && !win?.isDestroyed()) win.webContents.send('input-error','Tuş dinleyicisi durdu. Efekti yeniden başlatın.'); } if (!ready) reject(new Error('Tuş dinleyicisi Windows tarafından başlatılamadı.')); });
  });
}
app.whenReady().then(() => {
  const ses = session.defaultSession;
  ses.on('select-hid-device', (event, details, callback) => {
    event.preventDefault();
    const devices = details.deviceList.filter(d => d.vendorId === 0x373b && ids.includes(d.productId));
    callback(devices.length === 1 ? devices[0].deviceId : '');
  });
  ses.setPermissionCheckHandler((contents, permission) => contents === win?.webContents && contents.getURL() === page && permission === 'hid');
  ses.setDevicePermissionHandler(details => details.deviceType === 'hid' && details.device.vendorId === 0x373b && ids.includes(details.device.productId));
  win = new BrowserWindow({ title: 'RS7 Işık', width: 1180, height: 850, autoHideMenuBar: true, webPreferences: { preload: path.join(__dirname,'preload.cjs'), contextIsolation: true, nodeIntegration: false, sandbox: true, backgroundThrottling: false } });
  win.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
  win.webContents.on('will-navigate', (event, url) => { if (url !== page) event.preventDefault(); });
  win.webContents.on('render-process-gone', () => stopInput());
  ipcMain.handle('reactive', async (event, enabled) => {
    if (event.sender !== win.webContents || event.senderFrame !== win.webContents.mainFrame) throw new Error('Geçersiz istek.');
    if (!enabled) { stopInput(); return; }
    await startInput();
  });
  ipcMain.on('close-ready', event => { if (event.sender === win.webContents) { clearTimeout(closeTimer); allowClose = true; win.close(); } });
  win.on('close', event => { stopInput(); if (!allowClose) { event.preventDefault(); win.webContents.send('prepare-close'); closeTimer ??= setTimeout(() => { allowClose = true; win.close(); }, 2500); } });
  win.loadFile(path.join(__dirname,'..','index.html'));
});
app.on('window-all-closed', () => app.quit());
app.on('will-quit', stopInput);
