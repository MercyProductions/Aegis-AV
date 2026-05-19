import type { LucideIcon } from 'lucide-react';
import { StatusChip } from './StatusChip';

export function PageHeader({
  label,
  title,
  subtitle,
  icon: Icon,
  status
}: {
  label: string;
  title: string;
  subtitle: string;
  icon: LucideIcon;
  status?: string;
}) {
  return (
    <div className="col-span-12 mb-1 flex flex-wrap items-end justify-between gap-4">
      <div>
        <div className="flex items-center gap-3 text-sm font-medium text-aegis-green">
          <Icon className="h-4 w-4" />
          {label}
        </div>
        <h1 className="mt-3 text-[42px] font-bold leading-none tracking-normal text-white">{title}</h1>
        <p className="mt-4 max-w-3xl text-[16px] leading-7 text-white/62">{subtitle}</p>
      </div>
      {status && <StatusChip label={status} tone="good" pulse />}
    </div>
  );
}
