import { app, BrowserWindow, ipcMain } from 'electron';
import { execFile, spawn, type ChildProcess } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
let guardProcess: ChildProcess | undefined;
let mainWindow: BrowserWindow | undefined;

interface AgentCommandResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  error?: string;
}

function findAgentExecutable() {
  const candidates = [
    process.env.AEGIS_AGENT_EXE,
    path.resolve(process.resourcesPath, 'bin', 'aegis-agent.exe'),
    path.resolve(app.getAppPath(), '..', '..', 'target', 'release', 'aegis-agent.exe'),
    path.resolve(__dirname, '..', '..', '..', 'target', 'release', 'aegis-agent.exe'),
    path.resolve(process.cwd(), 'target', 'release', 'aegis-agent.exe'),
    path.resolve(process.cwd(), '..', '..', 'target', 'release', 'aegis-agent.exe')
  ].filter((candidate): candidate is string => Boolean(candidate));

  const found = candidates.find((candidate) => fs.existsSync(candidate));
  if (!found) {
    throw new Error('aegis-agent.exe was not found. Build it with: cargo build --release -p aegis-agent');
  }
  return found;
}

function runtimeRootFromAgent(agentPath: string) {
  if (agentPath.toLowerCase().startsWith(process.resourcesPath.toLowerCase())) {
    return process.resourcesPath;
  }

  return path.resolve(path.dirname(agentPath), '..', '..');
}

function runAgent(args: string[]): Promise<AgentCommandResult> {
  const agentPath = findAgentExecutable();
  return new Promise((resolve) => {
    execFile(
      agentPath,
      args,
      { cwd: runtimeRootFromAgent(agentPath), windowsHide: true },
      (error, stdout, stderr) => {
        resolve({
          ok: !error,
          stdout,
          stderr,
          error: error ? error.message : undefined
        });
      }
    );
  });
}

async function agentStatus() {
  const agentPath = findAgentExecutable();
  const result = await runAgent(['status', '--json']);
  let status: unknown = undefined;
  if (result.ok) {
    status = JSON.parse(result.stdout);
  }
  return {
    ...result,
    agentPath,
    guardRunning: Boolean(guardProcess && !guardProcess.killed && guardProcess.exitCode === null),
    status
  };
}

function registerAgentIpc() {
  ipcMain.handle('agent:status', () => agentStatus());
  ipcMain.handle('agent:arm', async () => {
    await runAgent(['arm']);
    return agentStatus();
  });
  ipcMain.handle('agent:disarm', async () => {
    await runAgent(['disarm']);
    return agentStatus();
  });
  ipcMain.handle('agent:start', async () => {
    if (!guardProcess || guardProcess.killed || guardProcess.exitCode !== null) {
      const agentPath = findAgentExecutable();
      guardProcess = spawn(agentPath, ['run', '--arm'], {
        cwd: runtimeRootFromAgent(agentPath),
        windowsHide: true,
        stdio: 'ignore'
      });
      guardProcess.on('exit', () => {
        guardProcess = undefined;
      });
    }
    return agentStatus();
  });
  ipcMain.handle('agent:stop', async () => {
    if (guardProcess && !guardProcess.killed) {
      guardProcess.kill();
      guardProcess = undefined;
    }
    await runAgent(['disarm']);
    return agentStatus();
  });
}

function registerWindowIpc() {
  ipcMain.handle('window:minimize', (event) => {
    BrowserWindow.fromWebContents(event.sender)?.minimize();
  });
  ipcMain.handle('window:maximize', (event) => {
    const window = BrowserWindow.fromWebContents(event.sender);
    if (!window) {
      return;
    }
    if (window.isMaximized()) {
      window.unmaximize();
    } else {
      window.maximize();
    }
  });
  ipcMain.handle('window:close', (event) => {
    BrowserWindow.fromWebContents(event.sender)?.close();
  });
}

function createWindow() {
  const window = new BrowserWindow({
    width: 1600,
    height: 1000,
    minWidth: 1280,
    minHeight: 720,
    frame: false,
    transparent: true,
    backgroundColor: '#00000000',
    title: 'Aegis AntiVirus',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true
    }
  });
  mainWindow = window;

  const devUrl = process.env.AEGIS_UI_DEV_URL;
  if (devUrl) {
    void window.loadURL(devUrl);
  } else {
    void window.loadFile(path.join(__dirname, '../dist/index.html'));
  }

  window.on('closed', () => {
    if (mainWindow === window) {
      mainWindow = undefined;
    }
  });
}

void app.whenReady().then(() => {
  registerAgentIpc();
  registerWindowIpc();
  createWindow();
});

app.on('window-all-closed', () => {
  if (guardProcess && !guardProcess.killed) {
    guardProcess.kill();
    guardProcess = undefined;
  }
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
