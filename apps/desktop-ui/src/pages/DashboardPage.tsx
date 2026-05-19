import { Activity, ArchiveRestore, Database, Gauge, ShieldAlert, ShieldCheck } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { motion } from 'framer-motion';
import { MetricTile } from '../components/MetricTile';
import { HeroProtectionCard } from '../widgets/HeroProtectionCard';
import { LiveEventFeed } from '../widgets/LiveEventFeed';
import { ProtectionModules } from '../widgets/ProtectionModules';
import { ScanStatusPanel } from '../widgets/ScanStatusPanel';
import { SystemHealthCard } from '../widgets/SystemHealthCard';
import { ThreatActivityChart } from '../widgets/ThreatActivityChart';
import { staggerContainer } from '../animations/motion';
import { useSecurityStore } from '../store/securityStore';

export function DashboardPage() {
  const status = useSecurityStore((state) => state.agentResponse?.status);

  const metrics = [
    {
      icon: Gauge,
      label: 'Protection Score',
      value: status?.armed === false ? '78' : '100',
      detail: status?.armed === false ? 'Realtime layer paused' : 'All core shields active',
      tone: status?.armed === false ? 'warning' : 'good'
    },
    {
      icon: ShieldCheck,
      label: 'Active Shields',
      value: status?.armed === false ? '4' : '5',
      detail: 'Realtime, web, firewall, ransomware, privacy',
      tone: 'good'
    },
    {
      icon: ArchiveRestore,
      label: 'Quarantine',
      value: String(status?.last_scan_threats ?? 0),
      detail: 'No permanent deletion by default',
      tone: (status?.last_scan_threats ?? 0) > 0 ? 'warning' : 'neutral'
    },
    {
      icon: Database,
      label: 'Definitions',
      value: '2026.05',
      detail: 'Verified local signature metadata',
      tone: 'neutral'
    }
  ] as const;

  return (
    <motion.div variants={staggerContainer} initial="initial" animate="animate" className="mx-auto grid max-w-[1500px] grid-cols-12 gap-6">
      <div className="col-span-12">
        <div className="mb-7 flex flex-wrap items-end justify-between gap-4">
          <div>
            <div className="text-sm font-medium text-white/58">
              Real-time protection is <span className="text-aegis-green underline decoration-aegis-green/40 underline-offset-4">active</span> and your system is <span className="text-aegis-green">secure</span>.
            </div>
            <h1 className="mt-2 text-[42px] font-bold leading-none tracking-normal text-white">Dashboard</h1>
          </div>
          <div className="rounded-full border border-white/[0.06] bg-white/[0.035] px-4 py-2 text-sm text-white/52">
            Local-first endpoint command center
          </div>
        </div>
      </div>

      <HeroProtectionCard />
      <ProtectionModules />

      <div className="col-span-12 grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
        {metrics.map((metric) => (
          <MetricTile key={metric.label} {...metric} />
        ))}
      </div>

      <ThreatActivityChart />
      <ScanStatusPanel />
      <SystemHealthCard />
      <LiveEventFeed />

      <div className="col-span-12 grid gap-4 xl:grid-cols-[1fr_1fr]">
        <CompactPanel
          title="Threat Intelligence"
          items={[
            ['Signature version', '2026.05.18.1'],
            ['Rule pack', 'windows-rules.safe'],
            ['Common suspicious location', 'Downloads'],
            ['Blocked event type', 'Child shell launch']
          ]}
          icon={ShieldAlert}
        />
        <CompactPanel
          title="System Health"
          items={[
            ['CPU policy', 'Balanced'],
            ['Battery saver', 'Ready'],
            ['Hash cache', 'Enabled'],
            ['Large-file rule', 'Skip with audit event']
          ]}
          icon={Activity}
        />
      </div>
    </motion.div>
  );
}

function CompactPanel({
  title,
  items,
  icon: Icon
}: {
  title: string;
  items: Array<[string, string]>;
  icon: LucideIcon;
}) {
  return (
    <section className="glass-card p-5">
      <div className="mb-4 flex items-center gap-3">
        <Icon className="h-5 w-5 text-aegis-green" />
        <h2 className="text-[18px] font-semibold text-white">{title}</h2>
      </div>
      <div className="grid gap-3 sm:grid-cols-2">
        {items.map(([label, value]) => (
          <div key={label} className="rounded-xl border border-white/[0.055] bg-white/[0.025] p-3">
            <div className="text-xs uppercase tracking-[0.14em] text-white/35">{label}</div>
            <div className="mt-2 text-sm font-medium text-white/72">{value}</div>
          </div>
        ))}
      </div>
    </section>
  );
}
