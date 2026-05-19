import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('aegis', {
  version: '0.1.2',
  agent: {
    status: () => ipcRenderer.invoke('agent:status'),
    arm: () => ipcRenderer.invoke('agent:arm'),
    disarm: () => ipcRenderer.invoke('agent:disarm'),
    start: () => ipcRenderer.invoke('agent:start'),
    stop: () => ipcRenderer.invoke('agent:stop')
  },
  scanner: {
    scan: (options: { profile: string; target?: string }) => ipcRenderer.invoke('scanner:scan', options),
    browseFolder: () => ipcRenderer.invoke('scanner:browse-folder')
  },
  windowControls: {
    minimize: () => ipcRenderer.invoke('window:minimize'),
    maximize: () => ipcRenderer.invoke('window:maximize'),
    close: () => ipcRenderer.invoke('window:close')
  }
});
