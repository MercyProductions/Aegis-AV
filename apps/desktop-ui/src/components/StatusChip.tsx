interface StatusChipProps {
  label: string;
  tone?: 'good' | 'warning' | 'danger' | 'neutral';
  pulse?: boolean;
}

const toneClasses = {
  good: 'border-aegis-green/25 bg-aegis-green/10 text-aegis-green',
  warning: 'border-aegis-amber/25 bg-aegis-amber/10 text-aegis-amber',
  danger: 'border-aegis-red/25 bg-aegis-red/10 text-aegis-red',
  neutral: 'border-white/[0.08] bg-white/[0.045] text-white/70'
};

const dotClasses = {
  good: 'bg-aegis-green shadow-[0_0_18px_rgba(108,255,108,0.65)]',
  warning: 'bg-aegis-amber shadow-[0_0_18px_rgba(255,184,77,0.45)]',
  danger: 'bg-aegis-red shadow-[0_0_18px_rgba(255,77,77,0.45)]',
  neutral: 'bg-white/55'
};

export function StatusChip({ label, tone = 'neutral', pulse = false }: StatusChipProps) {
  return (
    <span className={`inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-semibold ${toneClasses[tone]}`}>
      <span className={`h-1.5 w-1.5 rounded-full ${dotClasses[tone]} ${pulse ? 'animate-pulse' : ''}`} />
      {label}
    </span>
  );
}
