import { useEffect } from 'react';
import { useSecurityStore } from '../store/securityStore';

export function useAgentStatus() {
  const refreshAgent = useSecurityStore((state) => state.refreshAgent);

  useEffect(() => {
    void refreshAgent();
    const timer = window.setInterval(() => {
      void refreshAgent();
    }, 10000);
    return () => window.clearInterval(timer);
  }, [refreshAgent]);
}
