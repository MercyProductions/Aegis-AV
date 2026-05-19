import type { LucideIcon } from 'lucide-react';
import { motion } from 'framer-motion';

export interface MetricTileProps {
  icon: LucideIcon;
  label: string;
  value: string;
  detail: string;
  tone?: 'good' | 'warning' | 'danger' | 'neutral';
}

const toneClasses = {
  good: 'text-aegis-green',
  warning: 'text-aegis-amber',
  danger: 'text-aegis-red',
  neutral: 'text-white/68'
};

export function MetricTile({ icon: Icon, label, value, detail, tone = 'neutral' }: MetricTileProps) {
  return (
    <motion.article
      whileHover={{ y: -3, scale: 1.01 }}
      className="rounded-2xl border border-white/[0.06] bg-white/[0.035] p-4 transition-colors hover:border-aegis-green/30"
    >
      <div className="flex items-center gap-3">
        <div className="grid h-9 w-9 place-items-center rounded-xl border border-white/[0.08] bg-black/20">
          <Icon className={`h-4 w-4 ${toneClasses[tone]}`} />
        </div>
        <span className="text-xs font-medium uppercase tracking-[0.16em] text-white/45">{label}</span>
      </div>
      <div className="mt-4 text-[28px] font-bold leading-none text-white">{value}</div>
      <div className="mt-2 text-[13px] leading-5 text-white/56">{detail}</div>
    </motion.article>
  );
}
