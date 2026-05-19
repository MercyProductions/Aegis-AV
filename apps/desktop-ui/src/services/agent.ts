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

export type ScanProfile = 'quick' | 'full' | 'deep' | 'custom';

export interface FileMetadata {
  size_bytes: number;
  extension?: string | null;
  mime_guess?: string | null;
  modified_unix_seconds?: number | null;
}

export interface HeuristicFinding {
  rule_id: string;
  description: string;
  score: number;
}

export interface ScanResult {
  path: string;
  verdict: 'clean' | 'suspicious' | 'malicious' | 'skipped' | 'error';
  detection_name?: string | null;
  confidence_score: number;
  matched_rule?: string | null;
  sha256?: string | null;
  file_metadata?: FileMetadata | null;
  scan_duration_ms: number;
  heuristics: HeuristicFinding[];
  errors: string[];
}

export interface ScanSummary {
  target: string;
  profile: ScanProfile;
  files_scanned: number;
  files_skipped: number;
  errors: number;
  threats_found: number;
  suspicious_found: number;
  duration_ms: number;
  results: ScanResult[];
}

export interface ScanResponse {
  ok: boolean;
  stdout: string;
  stderr: string;
  error?: string;
  scannerPath?: string;
  summary?: ScanSummary;
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
  scanner?: {
    scan: (options: { profile: ScanProfile; target?: string }) => Promise<ScanResponse>;
    browseFolder: () => Promise<string | null>;
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
