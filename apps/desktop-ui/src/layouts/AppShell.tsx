import { motion } from 'framer-motion';
import type { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { TopBar } from './TopBar';
import { useAgentStatus } from '../hooks/useAgentStatus';

export function AppShell({ children }: { children: ReactNode }) {
  useAgentStatus();

  return (
    <main className="aegis-window flex h-screen overflow-hidden rounded-[24px] border border-white/[0.08] bg-aegis-bg text-white shadow-[0_30px_140px_rgba(0,0,0,0.68)]">
      <Sidebar />
      <section className="relative flex min-w-0 flex-1 flex-col overflow-hidden">
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_58%_0%,rgba(108,255,108,0.10),transparent_32%),radial-gradient(circle_at_92%_12%,rgba(52,208,88,0.08),transparent_28%)]" />
        <TopBar />
        <motion.div
          key="workspace"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.35 }}
          className="relative min-h-0 flex-1 overflow-auto px-8 py-7"
        >
          {children}
        </motion.div>
      </section>
    </main>
  );
}
