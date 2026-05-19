import { Activity, Clock3, Database, Play, Power, Radar, RotateCw, ShieldCheck, ShieldOff } from 'lucide-react';
import { motion } from 'framer-motion';
import { GlassCard } from '../components/GlassCard';
import { StatusChip } from '../components/StatusChip';
import { useSecurityStore } from '../store/securityStore';

function formatScanTime(value: number | null | undefined) {
  if (!value) {
    return 'No scan yet';
  }
  return new Intl.DateTimeFormat(undefined, {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit'
  }).format(new Date(value * 1000));
}

export function HeroProtectionCard() {
  const status = useSecurityStore((state) => state.agentResponse?.status);
  const guardRunning = useSecurityStore((state) => state.agentResponse?.guardRunning);
  const message = useSecurityStore((state) => state.agentMessage);
  const busy = useSecurityStore((state) => state.agentBusy);
  const startGuard = useSecurityStore((state) => state.startGuard);
  const armGuard = useSecurityStore((state) => state.armGuard);
  const disarmGuard = useSecurityStore((state) => state.disarmGuard);
  const refreshAgent = useSecurityStore((state) => state.refreshAgent);
  const protectedState = status?.armed !== false;

  return (
    <GlassCard className="relative col-span-12 min-h-[430px] overflow-hidden p-8 xl:col-span-7">
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_28%_36%,rgba(108,255,108,0.18),transparent_30%)]" />
      <div className="scan-line pointer-events-none absolute inset-0 opacity-35" />

      <div className="relative grid h-full grid-cols-[260px_minmax(0,1fr)] items-center gap-10">
        <div className="relative grid place-items-center">
          <motion.div
            animate={{ scale: [1, 1.035, 1], opacity: [0.76, 1, 0.76] }}
            transition={{ duration: 3.6, repeat: Infinity, ease: 'easeInOut' }}
            className="absolute h-[245px] w-[245px] rounded-full border border-aegis-green/30"
          />
          <motion.div
            animate={{ rotate: 360 }}
            transition={{ duration: 18, repeat: Infinity, ease: 'linear' }}
            className="absolute h-[208px] w-[208px] rounded-full border border-aegis-green/20 border-t-aegis-green/80"
          />
          <motion.div
            animate={{ rotate: -360 }}
            transition={{ duration: 24, repeat: Infinity, ease: 'linear' }}
            className="absolute h-[150px] w-[150px] rounded-full border border-aegis-green/15 border-b-aegis-green/60"
          />
          <div className="radar-disc">
            <ShieldCheck className="h-24 w-24 text-aegis-green drop-shadow-[0_0_28px_rgba(108,255,108,0.45)]" strokeWidth={1.65} />
          </div>
        </div>

        <div className="min-w-0">
          <div className="flex flex-wrap items-center gap-3">
            <StatusChip label={protectedState ? 'Protected' : 'Paused'} tone={protectedState ? 'good' : 'warning'} pulse />
            <StatusChip label={guardRunning ? 'Guard process running' : 'Desktop guard ready'} tone="neutral" />
          </div>
          <h1 className="mt-7 text-[42px] font-bold leading-tight tracking-normal text-white">
            {protectedState ? 'You Are Protected' : 'Protection Paused'}
          </h1>
          <p className="mt-3 max-w-xl text-[16px] leading-7 text-white/72">
            Aegis is watching key folders, scanning safe test signatures, and keeping local controls transparent.
          </p>

          <div className="mt-7 grid max-w-xl gap-4 text-sm text-white/70">
            <div className="hero-stat-row">
              <Activity className="h-4 w-4 text-aegis-green" />
              <span>Real-Time Protection</span>
              <strong className={protectedState ? 'text-aegis-green' : 'text-aegis-amber'}>{protectedState ? 'ON' : 'OFF'}</strong>
            </div>
            <div className="hero-stat-row">
              <Clock3 className="h-4 w-4 text-white/55" />
              <span>Last Scan</span>
              <strong>{formatScanTime(status?.last_scan_unix_seconds)}</strong>
            </div>
            <div className="hero-stat-row">
              <ShieldCheck className="h-4 w-4 text-white/55" />
              <span>Threats Blocked</span>
              <strong>{status?.last_scan_threats ?? 0}</strong>
            </div>
            <div className="hero-stat-row">
              <Database className="h-4 w-4 text-white/55" />
              <span>Definition Version</span>
              <strong>2026.05.18.1</strong>
            </div>
          </div>

          <div className="mt-8 flex flex-wrap items-center gap-4">
            <button className="primary-button min-w-[170px]" disabled={busy} onClick={() => void startGuard()}>
              <Radar className="h-4 w-4" /> Quick Scan
            </button>
            <button className="secondary-button min-w-[150px]" disabled={busy} onClick={() => void refreshAgent()}>
              <RotateCw className="h-4 w-4" /> Advanced Scan
            </button>
            <div className="h-8 w-px bg-white/[0.08]" />
            <button className="icon-text-button" disabled={busy} onClick={() => void armGuard()}>
              <Power className="h-4 w-4" /> Arm
            </button>
            <button className="icon-text-button" disabled={busy} onClick={() => void disarmGuard()}>
              <ShieldOff className="h-4 w-4" /> Disarm
            </button>
          </div>

          <div className="mt-4 flex items-center gap-2 text-xs text-white/45">
            <Play className="h-3.5 w-3.5 text-aegis-green" />
            {message}
          </div>
        </div>
      </div>
    </GlassCard>
  );
}
