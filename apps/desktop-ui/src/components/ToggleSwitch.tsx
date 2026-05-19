import { motion } from 'framer-motion';

interface ToggleSwitchProps {
  checked: boolean;
  onChange?: () => void;
  label: string;
}

export function ToggleSwitch({ checked, onChange, label }: ToggleSwitchProps) {
  return (
    <button
      aria-label={label}
      aria-pressed={checked}
      onClick={onChange}
      className={`relative h-7 w-12 rounded-full border p-1 transition-colors ${
        checked ? 'border-aegis-green/40 bg-aegis-green/20' : 'border-white/10 bg-white/5'
      }`}
    >
      <motion.span
        animate={{ x: checked ? 20 : 0 }}
        transition={{ type: 'spring', stiffness: 420, damping: 28 }}
        className={`block h-5 w-5 rounded-full ${checked ? 'bg-aegis-green shadow-[0_0_20px_rgba(108,255,108,0.55)]' : 'bg-white/45'}`}
      />
    </button>
  );
}
