import { Fingerprint, Globe2, LockKeyhole, Network, ShieldCheck } from 'lucide-react';
import { GlassCard } from '../components/GlassCard';
import { ToggleSwitch } from '../components/ToggleSwitch';
import { useSecurityStore } from '../store/securityStore';

const modules = [
  {
    id: 'realtime',
    name: 'Real-Time Protection',
    description: 'Monitors system activity in real time',
    icon: ShieldCheck
  },
  {
    id: 'web',
    name: 'Web Protection',
    description: 'Blocks dangerous sites and downloads',
    icon: Globe2
  },
  {
    id: 'firewall',
    name: 'Firewall',
    description: 'Shows process network activity',
    icon: Network
  },
  {
    id: 'ransomware',
    name: 'Ransomware Shield',
    description: 'Protects important folders from mass changes',
    icon: LockKeyhole
  },
  {
    id: 'privacy',
    name: 'Privacy Protection',
    description: 'Keeps telemetry optional and local-first',
    icon: Fingerprint
  }
];

export function ProtectionModules() {
  const status = useSecurityStore((state) => state.agentResponse?.status);
  const armGuard = useSecurityStore((state) => state.armGuard);
  const disarmGuard = useSecurityStore((state) => state.disarmGuard);
  const armed = status?.armed !== false;

  return (
    <GlassCard className="col-span-12 p-6 xl:col-span-5" delay={0.05}>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-[18px] font-semibold text-white">Protection Summary</h2>
        <button className="text-sm text-aegis-green transition hover:text-white">View full report</button>
      </div>

      <div className="divide-y divide-white/[0.06]">
        {modules.map((item) => {
          const Icon = item.icon;
          const checked = item.id === 'realtime' ? armed : true;
          return (
            <div key={item.id} className="grid grid-cols-[42px_minmax(0,1fr)_auto_auto] items-center gap-4 py-4">
              <div className="grid h-10 w-10 place-items-center rounded-xl border border-white/[0.06] bg-white/[0.035]">
                <Icon className="h-5 w-5 text-white/82" />
              </div>
              <div className="min-w-0">
                <div className="font-medium text-white">{item.name}</div>
                <div className="mt-1 text-[13px] text-white/52">{item.description}</div>
              </div>
              <div className="flex items-center gap-2 text-xs font-semibold text-aegis-green">
                ON <span className="h-2 w-2 rounded-full bg-aegis-green shadow-[0_0_16px_rgba(108,255,108,0.75)]" />
              </div>
              <ToggleSwitch
                label={`${item.name} toggle`}
                checked={checked}
                onChange={item.id === 'realtime' ? () => void (armed ? disarmGuard() : armGuard()) : undefined}
              />
            </div>
          );
        })}
      </div>
    </GlassCard>
  );
}
