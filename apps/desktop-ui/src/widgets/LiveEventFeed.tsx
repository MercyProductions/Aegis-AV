import { ShieldCheck } from 'lucide-react';
import { GlassCard } from '../components/GlassCard';

const events = [
  { time: '12:48', text: 'Guard status synchronized with desktop agent', level: 'info' },
  { time: '12:42', text: 'Signature metadata verified for local EICAR-safe rules', level: 'good' },
  { time: '12:36', text: 'Hash cache skipped 412 unchanged files', level: 'info' },
  { time: '12:31', text: 'Ransomware shield watched protected folders', level: 'good' }
];

export function LiveEventFeed() {
  return (
    <GlassCard className="col-span-12 p-5" delay={0.22}>
      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="grid h-9 w-9 place-items-center rounded-xl border border-aegis-green/20 bg-aegis-green/10">
            <ShieldCheck className="h-4 w-4 text-aegis-green" />
          </div>
          <div className="text-sm text-white/62">Tip: Schedule regular scans to keep your system safe.</div>
        </div>
        <div className="hidden items-center gap-8 text-sm text-white/55 lg:flex">
          <span>Next scheduled scan: May 21, 2026 2:00 AM</span>
          <button className="text-aegis-green transition hover:text-white">Manage Schedule</button>
        </div>
      </div>
      <div className="mt-4 grid gap-2 md:grid-cols-4">
        {events.map((event) => (
          <div key={`${event.time}-${event.text}`} className="rounded-xl border border-white/[0.05] bg-white/[0.025] px-3 py-2">
            <div className="text-xs text-white/35">{event.time}</div>
            <div className={event.level === 'good' ? 'mt-1 text-xs text-aegis-green' : 'mt-1 text-xs text-white/58'}>{event.text}</div>
          </div>
        ))}
      </div>
    </GlassCard>
  );
}
