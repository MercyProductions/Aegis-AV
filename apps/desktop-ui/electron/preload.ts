import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('aegis', {
  version: '0.1.0',
  agent: {
    status: () => ipcRenderer.invoke('agent:status'),
    arm: () => ipcRenderer.invoke('agent:arm'),
    disarm: () => ipcRenderer.invoke('agent:disarm'),
    start: () => ipcRenderer.invoke('agent:start'),
    stop: () => ipcRenderer.invoke('agent:stop')
  },
  windowControls: {
    minimize: () => ipcRenderer.invoke('window:minimize'),
    maximize: () => ipcRenderer.invoke('window:maximize'),
    close: () => ipcRenderer.invoke('window:close')
  }
});
