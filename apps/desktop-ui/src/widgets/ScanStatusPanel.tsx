import { CheckCircle2, ShieldCheck, ShieldQuestion } from 'lucide-react';
import { GlassCard } from '../components/GlassCard';

const scans = [
  { name: 'Quick Scan', state: 'Completed', time: 'Today, 8:42 AM', result: 'No threats found', active: true },
  { name: 'Full System Scan', state: 'Not completed', time: 'Ready', result: 'Awaiting run', active: false },
  { name: 'Custom Scan', state: 'Not completed', time: 'Ready', result: 'Choose folder', active: false }
];

export function ScanStatusPanel() {
  return (
    <GlassCard className="col-span-12 p-6 xl:col-span-4" delay={0.14}>
      <div className="mb-5 flex items-center justify-between">
        <h2 className="text-[18px] font-semibold text-white">Scan Status</h2>
        <button className="text-sm text-aegis-green transition hover:text-white">View all scans</button>
      </div>

      <div className="space-y-3">
        {scans.map((scan) => (
          <button
            key={scan.name}
            className={`w-full rounded-2xl border p-4 text-left transition hover:-translate-y-0.5 ${
              scan.active ? 'border-aegis-green/25 bg-aegis-green/[0.055]' : 'border-white/[0.06] bg-white/[0.025]'
            }`}
          >
            <div className="grid grid-cols-[34px_minmax(0,1fr)_auto] gap-3">
              {scan.active ? <ShieldCheck className="h-5 w-5 text-aegis-green" /> : <ShieldQuestion className="h-5 w-5 text-white/48" />}
              <div>
                <div className="font-medium text-white">{scan.name}</div>
                <div className="mt-1 text-sm text-white/45">{scan.state}</div>
              </div>
              <div className="text-right">
                <div className="text-sm text-white/65">{scan.time}</div>
                <div className={scan.active ? 'mt-4 text-sm text-aegis-green' : 'mt-4 text-sm text-white/42'}>{scan.result}</div>
              </div>
            </div>
          </button>
        ))}
      </div>

      <button className="primary-button mt-5 w-full">
        <CheckCircle2 className="h-4 w-4" /> Run New Scan
      </button>
    </GlassCard>
  );
}
