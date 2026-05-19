import { ShieldCheck } from 'lucide-react';
import { motion } from 'framer-motion';

export function BrandMark({ expanded }: { expanded: boolean }) {
  return (
    <div className="flex h-[86px] items-center gap-4 px-5">
      <motion.div
        className="relative grid h-12 w-12 shrink-0 place-items-center rounded-2xl border border-white/10 bg-aegis-elevated shadow-glow"
        whileHover={{ scale: 1.04 }}
      >
        <div className="absolute inset-1 rounded-xl border border-aegis-green/30" />
        <ShieldCheck className="relative h-7 w-7 text-aegis-green" strokeWidth={1.8} />
      </motion.div>
      <motion.div
        animate={{ opacity: expanded ? 1 : 0, x: expanded ? 0 : -10 }}
        className="min-w-0 overflow-hidden"
      >
        <div className="tracking-[0.42em] text-[15px] font-semibold uppercase text-white">Aegis</div>
        <div className="mt-1 whitespace-nowrap text-[11px] uppercase tracking-[0.34em] text-white/55">
          Antivirus
        </div>
      </motion.div>
    </div>
  );
}
