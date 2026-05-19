import { create } from 'zustand';
import { getAegisBridge, type AgentResponse, type ScanProfile, type ScanResponse } from '../services/agent';

export type PageId =
  | 'dashboard'
  | 'scan'
  | 'realtime'
  | 'threats'
  | 'firewall'
  | 'web'
  | 'ransomware'
  | 'privacy'
  | 'quarantine'
  | 'reports'
  | 'settings';

interface SecurityState {
  page: PageId;
  sidebarExpanded: boolean;
  agentResponse: AgentResponse | null;
  agentMessage: string;
  agentBusy: boolean;
  scanProfile: ScanProfile;
  customScanTarget: string;
  scanResponse: ScanResponse | null;
  scanMessage: string;
  scanBusy: boolean;
  activePolicy: string;
  setPage: (page: PageId) => void;
  setSidebarExpanded: (expanded: boolean) => void;
  setScanProfile: (profile: ScanProfile) => void;
  setCustomScanTarget: (target: string) => void;
  setActivePolicy: (policy: string) => void;
  refreshAgent: () => Promise<void>;
  startGuard: () => Promise<void>;
  armGuard: () => Promise<void>;
  disarmGuard: () => Promise<void>;
  stopGuard: () => Promise<void>;
  browseForCustomScanTarget: () => Promise<void>;
  runScan: (profile?: ScanProfile, target?: string) => Promise<void>;
}

async function runAgentAction(
  label: string,
  action: (() => Promise<AgentResponse>) | undefined,
  set: (partial: Partial<SecurityState>) => void
) {
  if (!action) {
    set({
      agentMessage: 'Open the Electron desktop app to control the local guard.',
      agentBusy: false
    });
    return;
  }

  set({ agentBusy: true, agentMessage: `${label}...` });
  try {
    const response = await action();
    set({
      agentResponse: response,
      agentMessage: response.ok ? `${label} complete` : response.error ?? `${label} failed`,
      agentBusy: false
    });
  } catch (error) {
    set({
      agentMessage: error instanceof Error ? error.message : `${label} failed`,
      agentBusy: false
    });
  }
}

export const useSecurityStore = create<SecurityState>((set) => ({
  page: 'dashboard',
  sidebarExpanded: false,
  agentResponse: null,
  agentMessage: 'Ready',
  agentBusy: false,
  scanProfile: 'quick',
  customScanTarget: '',
  scanResponse: null,
  scanMessage: 'Ready to scan',
  scanBusy: false,
  activePolicy: 'Balanced',
  setPage: (page) => set({ page }),
  setSidebarExpanded: (sidebarExpanded) => set({ sidebarExpanded }),
  setScanProfile: (scanProfile) => set({ scanProfile }),
  setCustomScanTarget: (customScanTarget) => set({ customScanTarget }),
  setActivePolicy: (activePolicy) => set({ activePolicy }),
  refreshAgent: async () => {
    const action = getAegisBridge()?.agent?.status;
    if (!action) {
      set({ agentMessage: 'Open the Electron desktop app to control the local guard.' });
      return;
    }

    try {
      const response = await action();
      set({
        agentResponse: response,
        agentMessage: response.ok ? 'Live status synced' : response.error ?? 'Status refresh failed'
      });
    } catch (error) {
      set({ agentMessage: error instanceof Error ? error.message : 'Status refresh failed' });
    }
  },
  startGuard: () => runAgentAction('Start guard', getAegisBridge()?.agent?.start, set),
  armGuard: () => runAgentAction('Arm guard', getAegisBridge()?.agent?.arm, set),
  disarmGuard: () => runAgentAction('Disarm guard', getAegisBridge()?.agent?.disarm, set),
  stopGuard: () => runAgentAction('Stop guard', getAegisBridge()?.agent?.stop, set),
  browseForCustomScanTarget: async () => {
    const browse = getAegisBridge()?.scanner?.browseFolder;
    if (!browse) {
      set({ scanMessage: 'Open the Electron desktop app to choose folders.' });
      return;
    }
    const target = await browse();
    if (target) {
      set({ customScanTarget: target, scanProfile: 'custom', scanMessage: 'Custom scan folder selected' });
    }
  },
  runScan: async (profile, target) => {
    const scanner = getAegisBridge()?.scanner?.scan;
    const state = useSecurityStore.getState();
    const selectedProfile = profile ?? state.scanProfile;
    const selectedTarget = target ?? (selectedProfile === 'custom' ? state.customScanTarget : undefined);

    if (!scanner) {
      set({ scanMessage: 'Open the Electron desktop app to run the local Rust scanner.' });
      return;
    }

    set({
      scanBusy: true,
      scanProfile: selectedProfile,
      scanMessage: `${selectedProfile[0].toUpperCase()}${selectedProfile.slice(1)} scan running...`
    });
    try {
      const response = await scanner({ profile: selectedProfile, target: selectedTarget });
      set({
        scanResponse: response,
        scanMessage: response.ok
          ? `Scan complete: ${response.summary?.files_scanned ?? 0} files, ${response.summary?.threats_found ?? 0} threats`
          : response.error ?? 'Scan failed',
        scanBusy: false
      });
    } catch (error) {
      set({
        scanMessage: error instanceof Error ? error.message : 'Scan failed',
        scanBusy: false
      });
    }
  }
}));
