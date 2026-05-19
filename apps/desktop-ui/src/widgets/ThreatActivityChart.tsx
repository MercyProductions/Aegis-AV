import { ArchiveRestore, ShieldCheck, ShieldX } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts';
import { GlassCard } from '../components/GlassCard';

const data = [
  { day: 'May 12', blocked: 4 },
  { day: 'May 13', blocked: 7 },
  { day: 'May 14', blocked: 3 },
  { day: 'May 15', blocked: 8 },
  { day: 'May 16', blocked: 13 },
  { day: 'May 17', blocked: 7 },
  { day: 'May 18', blocked: 10 },
  { day: 'May 19', blocked: 9 },
  { day: 'May 20', blocked: 15 }
];

export function ThreatActivityChart() {
  return (
    <GlassCard className="col-span-12 p-6 xl:col-span-5" delay={0.1}>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-[18px] font-semibold text-white">Threat Activity</h2>
        <span className="text-sm text-white/45">Last 7 days</span>
      </div>

      <div className="h-[260px]">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data} margin={{ left: -20, right: 10, top: 10, bottom: 0 }}>
            <defs>
              <linearGradient id="threatActivity" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#6CFF6C" stopOpacity={0.34} />
                <stop offset="95%" stopColor="#6CFF6C" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid stroke="rgba(255,255,255,0.06)" vertical={false} />
            <XAxis dataKey="day" stroke="rgba(255,255,255,0.42)" tickLine={false} axisLine={false} tick={{ fontSize: 12 }} />
            <YAxis stroke="rgba(255,255,255,0.42)" tickLine={false} axisLine={false} tick={{ fontSize: 12 }} />
            <Tooltip
              cursor={{ stroke: 'rgba(108,255,108,0.28)' }}
              contentStyle={{
                background: '#101720',
                border: '1px solid rgba(255,255,255,0.08)',
                borderRadius: 14,
                color: '#fff'
              }}
            />
            <Area
              type="monotone"
              dataKey="blocked"
              stroke="#6CFF6C"
              strokeWidth={3}
              fill="url(#threatActivity)"
              style={{ filter: 'drop-shadow(0 0 12px rgba(108,255,108,0.45))' }}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      <div className="mt-5 grid grid-cols-3 overflow-hidden rounded-2xl border border-white/[0.06] bg-white/[0.025]">
        <Metric icon={ShieldCheck} value="24" label="Threats Blocked" />
        <Metric icon={ArchiveRestore} value="3" label="Files Quarantined" />
        <Metric icon={ShieldX} value="0" label="Threats Removed" />
      </div>
    </GlassCard>
  );
}

function Metric({ icon: Icon, value, label }: { icon: LucideIcon; value: string; label: string }) {
  return (
    <div className="border-r border-white/[0.06] p-4 last:border-r-0">
      <Icon className="mb-3 h-5 w-5 text-aegis-green" />
      <div className="text-2xl font-bold text-white">{value}</div>
      <div className="mt-1 text-xs text-white/55">{label}</div>
    </div>
  );
}
