import { AppShell } from './layouts/AppShell';
import { DashboardPage } from './pages/DashboardPage';
import { PlaceholderPage } from './pages/PlaceholderPage';
import { useSecurityStore, type PageId } from './store/securityStore';

const pageTitles: Record<PageId, string> = {
  dashboard: 'Dashboard',
  scan: 'Smart Scan',
  realtime: 'Real-Time Protection',
  threats: 'Threat Center',
  firewall: 'Firewall',
  web: 'Web Protection',
  ransomware: 'Ransomware Shield',
  privacy: 'Privacy',
  quarantine: 'Quarantine',
  reports: 'Reports',
  settings: 'Settings'
};

const pageSubtitles: Record<PageId, string> = {
  dashboard: 'Real-time protection is active and your system is secure.',
  scan: 'Launch quick, full, and custom scans from a focused command surface.',
  realtime: 'Monitor the live guard, watched folders, and protection modules.',
  threats: 'Review detections, risk evidence, and recommended actions.',
  firewall: 'See outbound activity and connection visibility in one place.',
  web: 'Inspect download protection, web events, and reputation signals.',
  ransomware: 'Track protected folders and suspicious file-change bursts.',
  privacy: 'Control local-first protection, telemetry, and trust settings.',
  quarantine: 'Review isolated files, metadata, restore, and exclusion actions.',
  reports: 'Export incident reports, scan summaries, and event timelines.',
  settings: 'Tune policies, profiles, exclusions, updates, and notifications.'
};

export function App() {
  const page = useSecurityStore((state) => state.page);

  return (
    <AppShell>
      {page === 'dashboard' ? (
        <DashboardPage />
      ) : (
        <PlaceholderPage title={pageTitles[page]} subtitle={pageSubtitles[page]} />
      )}
    </AppShell>
  );
}
