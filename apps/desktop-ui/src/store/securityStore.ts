import { create } from 'zustand';
import { getAegisBridge, type AgentResponse } from '../services/agent';

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
  setPage: (page: PageId) => void;
  setSidebarExpanded: (expanded: boolean) => void;
  refreshAgent: () => Promise<void>;
  startGuard: () => Promise<void>;
  armGuard: () => Promise<void>;
  disarmGuard: () => Promise<void>;
  stopGuard: () => Promise<void>;
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
  setPage: (page) => set({ page }),
  setSidebarExpanded: (sidebarExpanded) => set({ sidebarExpanded }),
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
  stopGuard: () => runAgentAction('Stop guard', getAegisBridge()?.agent?.stop, set)
}));
