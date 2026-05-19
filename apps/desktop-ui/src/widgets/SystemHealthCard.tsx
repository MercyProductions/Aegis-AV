import { Check, ShieldCheck } from 'lucide-react';
import { GlassCard } from '../components/GlassCard';

const checks = ['No security issues detected', 'All modules are up to date', 'Real-time protection active'];

export function SystemHealthCard() {
  return (
    <GlassCard className="col-span-12 p-6 xl:col-span-3" delay={0.18}>
      <h2 className="text-[18px] font-semibold text-white">System Protection</h2>
      <div className="mt-8 grid place-items-center">
        <div className="health-ring">
          <div className="grid h-[142px] w-[142px] place-items-center rounded-full bg-aegis-card text-center">
            <div>
              <div className="text-[36px] font-bold leading-none text-white">100%</div>
              <div className="mt-2 text-sm text-white/72">Secure</div>
            </div>
          </div>
        </div>
      </div>
      <div className="mt-8 text-center text-sm font-medium text-aegis-green">All protection layers are active</div>
      <div className="mt-6 space-y-4">
        {checks.map((check) => (
          <div key={check} className="flex items-center gap-3 text-sm text-white/62">
            <Check className="h-4 w-4 text-aegis-green" />
            <span>{check}</span>
          </div>
        ))}
      </div>
      <div className="mt-7 flex items-center gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.03] p-3">
        <ShieldCheck className="h-5 w-5 text-aegis-green" />
        <span className="text-sm text-white/58">Integrity checks verified</span>
      </div>
    </GlassCard>
  );
}
