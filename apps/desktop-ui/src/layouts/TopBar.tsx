import { Bell, Command, Cpu, Search, ShieldCheck } from 'lucide-react';
import { StatusChip } from '../components/StatusChip';
import { WindowControls } from '../components/WindowControls';
import { useSecurityStore } from '../store/securityStore';

export function TopBar() {
  const page = useSecurityStore((state) => state.page);
  const agentStatus = useSecurityStore((state) => state.agentResponse?.status);
  const protectedState = agentStatus?.armed !== false;

  return (
    <header className="drag-region flex h-[70px] shrink-0 items-center justify-between border-b border-white/[0.06] bg-[#0B1016]/55 px-8 backdrop-blur-2xl">
      <div className="no-drag flex w-[420px] items-center gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.035] px-4 py-3 text-white/52">
        <Search className="h-4 w-4" />
        <span className="text-sm">Search threats, hashes, files, reports</span>
        <Command className="ml-auto h-4 w-4 text-white/35" />
      </div>

      <div className="no-drag flex items-center gap-3">
        <StatusChip label={protectedState ? 'Live protection active' : 'Protection paused'} tone={protectedState ? 'good' : 'warning'} pulse />
        <StatusChip label="System health 100%" tone="neutral" />
        <button className="top-icon-button" title="System health">
          <Cpu className="h-4 w-4" />
        </button>
        <button className="top-icon-button relative" title="Notifications">
          <Bell className="h-4 w-4" />
          <span className="absolute right-2 top-2 h-2 w-2 rounded-full bg-aegis-green shadow-[0_0_14px_rgba(108,255,108,0.8)]" />
        </button>
        <div className="mx-2 h-7 w-px bg-white/[0.08]" />
        <div className="hidden items-center gap-2 rounded-full border border-white/[0.06] bg-white/[0.035] px-3 py-2 text-xs text-white/58 xl:flex">
          <ShieldCheck className="h-4 w-4 text-aegis-green" />
          {page === 'dashboard' ? 'Command workspace' : 'Secure workspace'}
        </div>
        <WindowControls />
      </div>
    </header>
  );
}
