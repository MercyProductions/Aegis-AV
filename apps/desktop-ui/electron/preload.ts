import { contextBridge } from 'electron';

contextBridge.exposeInMainWorld('aegis', {
  version: '0.1.0'
});
