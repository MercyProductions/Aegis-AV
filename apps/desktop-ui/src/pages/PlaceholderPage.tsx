import { Activity, ArrowRight, Gauge, LockKeyhole, ShieldCheck } from 'lucide-react';
import { GlassCard } from '../components/GlassCard';
import { MetricTile } from '../components/MetricTile';

export function PlaceholderPage({ title, subtitle }: { title: string; subtitle: string }) {
  return (
    <div className="mx-auto grid max-w-[1500px] grid-cols-12 gap-6">
      <div className="col-span-12">
        <div className="mb-7">
          <div className="text-sm font-medium text-aegis-green">Aegis Operations Workspace</div>
          <h1 className="mt-2 text-[42px] font-bold leading-none tracking-normal text-white">{title}</h1>
          <p className="mt-4 max-w-3xl text-[16px] leading-7 text-white/62">{subtitle}</p>
        </div>
      </div>

      <GlassCard className="col-span-12 min-h-[320px] p-8 xl:col-span-8">
        <div className="flex h-full flex-col justify-between gap-10">
          <div>
            <div className="inline-flex items-center gap-2 rounded-full border border-aegis-green/20 bg-aegis-green/10 px-3 py-1.5 text-xs font-semibold text-aegis-green">
              <ShieldCheck className="h-3.5 w-3.5" />
              Premium module surface
            </div>
            <h2 className="mt-8 max-w-2xl text-3xl font-bold leading-tight text-white">
              Built as a reusable security workspace, ready for deeper product logic.
            </h2>
            <p className="mt-4 max-w-2xl text-white/60">
              The shell, cards, status chips, toggles, metrics, and panels are modular so each Aegis page can mature without reworking the visual system.
            </p>
          </div>
          <button className="secondary-button w-fit">
            Open module details <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      </GlassCard>

      <div className="col-span-12 grid grid-cols-1 gap-4 md:grid-cols-3 xl:col-span-4">
        <MetricTile icon={Gauge} label="Readiness" value="92%" detail="Design system aligned" tone="good" />
        <MetricTile icon={Activity} label="Events" value="Live" detail="Ready for event bus feeds" tone="neutral" />
        <MetricTile icon={LockKeyhole} label="Trust" value="Local" detail="Transparent controls first" tone="good" />
      </div>
    </div>
  );
}
