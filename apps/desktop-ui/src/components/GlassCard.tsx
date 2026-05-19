import { motion, type HTMLMotionProps } from 'framer-motion';
import { cardMotion } from '../animations/motion';

interface GlassCardProps extends HTMLMotionProps<'section'> {
  delay?: number;
}

export function GlassCard({ className = '', delay = 0, children, ...props }: GlassCardProps) {
  return (
    <motion.section
      {...cardMotion}
      transition={{ ...cardMotion.transition, delay }}
      className={`glass-card ${className}`}
      {...props}
    >
      {children}
    </motion.section>
  );
}
