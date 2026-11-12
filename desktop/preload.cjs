const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld('desktop', {
  reactive: enabled => ipcRenderer.invoke('reactive', enabled === true),
  onKey: callback => { const handler = (_event, scan) => callback(scan); ipcRenderer.on('lighting-key', handler); return () => ipcRenderer.removeListener('lighting-key', handler); },
  onInputError: callback => { const handler = (_event, message) => callback(message); ipcRenderer.on('input-error', handler); return () => ipcRenderer.removeListener('input-error', handler); },
  onClose: callback => ipcRenderer.on('prepare-close', () => callback()),
  closed: () => ipcRenderer.send('close-ready')
});
