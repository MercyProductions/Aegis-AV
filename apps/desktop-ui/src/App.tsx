import { AppShell } from './layouts/AppShell';
import { DashboardPage } from './pages/DashboardPage';
import {
  FirewallPage,
  PrivacyPage,
  QuarantinePage,
  RansomwarePage,
  RealTimeProtectionPage,
  ReportsPage,
  SettingsPage,
  SmartScanPage,
  ThreatCenterPage,
  WebProtectionPage
} from './pages/OperationsPages';
import { useSecurityStore } from './store/securityStore';

export function App() {
  const page = useSecurityStore((state) => state.page);

  return (
    <AppShell>
      {page === 'dashboard' && <DashboardPage />}
      {page === 'scan' && <SmartScanPage />}
      {page === 'realtime' && <RealTimeProtectionPage />}
      {page === 'threats' && <ThreatCenterPage />}
      {page === 'firewall' && <FirewallPage />}
      {page === 'web' && <WebProtectionPage />}
      {page === 'ransomware' && <RansomwarePage />}
      {page === 'privacy' && <PrivacyPage />}
      {page === 'quarantine' && <QuarantinePage />}
      {page === 'reports' && <ReportsPage />}
      {page === 'settings' && <SettingsPage />}
    </AppShell>
  );
}
