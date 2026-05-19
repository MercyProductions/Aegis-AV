export interface AgentStatus {
  armed: boolean;
  interval_seconds: number;
  watched_paths: string[];
  last_scan_unix_seconds: number | null;
  last_scan_files: number;
  last_scan_suspicious: number;
  last_scan_threats: number;
  last_scan_errors: number;
  state_file?: string;
  log_file?: string;
}

export interface AgentResponse {
  ok: boolean;
  stdout: string;
  stderr: string;
  error?: string;
  agentPath?: string;
  guardRunning?: boolean;
  status?: AgentStatus;
}

export interface AegisBridge {
  version: string;
  agent?: {
    status: () => Promise<AgentResponse>;
    arm: () => Promise<AgentResponse>;
    disarm: () => Promise<AgentResponse>;
    start: () => Promise<AgentResponse>;
    stop: () => Promise<AgentResponse>;
  };
  windowControls?: {
    minimize: () => Promise<void>;
    maximize: () => Promise<void>;
    close: () => Promise<void>;
  };
}

declare global {
  interface Window {
    aegis?: AegisBridge;
  }
}

export function getAegisBridge() {
  return window.aegis;
}
