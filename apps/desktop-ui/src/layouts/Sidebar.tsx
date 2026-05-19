import {
  ArchiveRestore,
  FileText,
  Fingerprint,
  Globe2,
  LayoutDashboard,
  LockKeyhole,
  Network,
  Search,
  Settings,
  ShieldAlert,
  ShieldCheck
} from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { motion } from 'framer-motion';
import { BrandMark } from '../components/BrandMark';
import { useSecurityStore, type PageId } from '../store/securityStore';

interface NavItem {
  id: PageId;
  label: string;
  icon: LucideIcon;
}

const navItems: NavItem[] = [
  { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { id: 'scan', label: 'Smart Scan', icon: Search },
  { id: 'realtime', label: 'Real-Time Protection', icon: ShieldCheck },
  { id: 'threats', label: 'Threat Center', icon: ShieldAlert },
  { id: 'firewall', label: 'Firewall', icon: Network },
  { id: 'web', label: 'Web Protection', icon: Globe2 },
  { id: 'ransomware', label: 'Ransomware Shield', icon: LockKeyhole },
  { id: 'privacy', label: 'Privacy', icon: Fingerprint },
  { id: 'quarantine', label: 'Quarantine', icon: ArchiveRestore },
  { id: 'reports', label: 'Reports', icon: FileText },
  { id: 'settings', label: 'Settings', icon: Settings }
];

export function Sidebar() {
  const page = useSecurityStore((state) => state.page);
  const expanded = useSecurityStore((state) => state.sidebarExpanded);
  const setPage = useSecurityStore((state) => state.setPage);
  const setSidebarExpanded = useSecurityStore((state) => state.setSidebarExpanded);
  const agentStatus = useSecurityStore((state) => state.agentResponse?.status);

  return (
    <motion.aside
      onMouseEnter={() => setSidebarExpanded(true)}
      onMouseLeave={() => setSidebarExpanded(false)}
      animate={{ width: expanded ? 260 : 90 }}
      transition={{ type: 'spring', stiffness: 260, damping: 30 }}
      className="no-drag relative z-20 flex shrink-0 flex-col border-r border-white/[0.06] bg-[#060A0F]/96 shadow-[28px_0_90px_rgba(0,0,0,0.22)]"
    >
      <BrandMark expanded={expanded} />

      <nav className="mt-4 flex flex-1 flex-col gap-1 px-3">
        {navItems.map((item) => {
          const Icon = item.icon;
          const active = page === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setPage(item.id)}
              title={item.label}
              className={`group relative flex h-12 items-center gap-4 overflow-hidden rounded-2xl border px-4 text-left transition ${
                active
                  ? 'border-aegis-green/25 bg-aegis-green/10 text-white shadow-[0_0_28px_rgba(108,255,108,0.13)]'
                  : 'border-transparent text-white/68 hover:border-white/[0.06] hover:bg-white/[0.045] hover:text-white'
              }`}
            >
              {active && <span className="absolute left-0 h-7 w-0.5 rounded-r bg-aegis-green shadow-[0_0_18px_rgba(108,255,108,0.8)]" />}
              <Icon className={`h-5 w-5 shrink-0 transition ${active ? 'text-aegis-green' : 'group-hover:scale-105'}`} strokeWidth={1.75} />
              <motion.span
                animate={{ opacity: expanded ? 1 : 0, x: expanded ? 0 : -8 }}
                className="whitespace-nowrap text-[14px] font-medium"
              >
                {item.label}
              </motion.span>
            </button>
          );
        })}
      </nav>

      <div className="border-t border-white/[0.06] px-4 py-5">
        <div className="flex items-center gap-3 rounded-2xl border border-aegis-green/20 bg-aegis-green/10 p-3">
          <div className="grid h-10 w-10 shrink-0 place-items-center rounded-full border border-aegis-green/25 bg-aegis-green/10">
            <ShieldCheck className="h-5 w-5 text-aegis-green" />
          </div>
          <motion.div
            animate={{ opacity: expanded ? 1 : 0, x: expanded ? 0 : -8 }}
            className="min-w-0 overflow-hidden"
          >
            <div className="text-sm font-semibold text-aegis-green">
              {agentStatus?.armed === false ? 'Paused' : 'Protected'}
            </div>
            <div className="mt-0.5 whitespace-nowrap text-xs text-white/55">Your system is secure</div>
          </motion.div>
        </div>

        <motion.div
          animate={{ opacity: expanded ? 1 : 0, height: expanded ? 'auto' : 0 }}
          className="overflow-hidden"
        >
          <div className="mt-5 border-t border-white/[0.06] pt-4 text-xs leading-6 text-white/52">
            <div>Version 0.1.2</div>
            <div>Definitions: 2026.05.18.1</div>
            <button className="mt-2 text-aegis-green transition hover:text-white">Check for updates</button>
          </div>
        </motion.div>
      </div>
    </motion.aside>
  );
}
