import type { LucideIcon } from 'lucide-react';
import {
  Activity,
  ArchiveRestore,
  Bell,
  Building2,
  CheckCircle2,
  ClipboardCheck,
  Cloud,
  Cpu,
  Database,
  FileSearch,
  Gauge,
  History,
  Layers3,
  LockKeyhole,
  Radar,
  RefreshCw,
  Settings,
  ShieldCheck,
  ShieldAlert,
  ShieldPlus,
  Siren
} from 'lucide-react';

export type PageKey =
  | 'security'
  | 'deviceControl'
  | 'aegisCore'
  | 'systemGraph'
  | 'orchestration'
  | 'predictiveRisk'
  | 'aiOps'
  | 'commandOs'
  | 'sync'
  | 'observability'
  | 'workspaces'
  | 'visualIntel'
  | 'distributed'
  | 'sdk'
  | 'transparency'
  | 'premiumOs'
  | 'aegisOs'
  | 'operations'
  | 'reliability'
  | 'events'
  | 'automation'
  | 'alerts'
  | 'profiles'
  | 'knowledge'
  | 'assistant'
  | 'marketplace'
  | 'trust'
  | 'brand'
  | 'platform'
  | 'vision'
  | 'modules'
  | 'layers'
  | 'scan'
  | 'incidents'
  | 'behavior'
  | 'processes'
  | 'diagnostics'
  | 'network'
  | 'ransomware'
  | 'sandbox'
  | 'ai'
  | 'plugins'
  | 'tools'
  | 'score'
  | 'ecosystem'
  | 'quarantine'
  | 'intel'
  | 'enterprise'
  | 'release'
  | 'updates'
  | 'logs'
  | 'settings';

export interface NavItem {
  key: PageKey;
  label: string;
  icon: LucideIcon;
}

export const navItems: NavItem[] = [
  { key: 'security', label: 'Security', icon: ShieldCheck },
  { key: 'deviceControl', label: 'Device Control', icon: ShieldPlus },
  { key: 'aegisCore', label: 'AegisCore', icon: ShieldCheck },
  { key: 'systemGraph', label: 'System Graph', icon: Layers3 },
  { key: 'orchestration', label: 'Orchestrate', icon: RefreshCw },
  { key: 'predictiveRisk', label: 'Risk Engine', icon: Gauge },
  { key: 'aiOps', label: 'AI Ops', icon: Activity },
  { key: 'commandOs', label: 'Command OS', icon: Radar },
  { key: 'sync', label: 'Sync', icon: Cloud },
  { key: 'observability', label: 'Observability', icon: Cpu },
  { key: 'workspaces', label: 'Workspaces', icon: Database },
  { key: 'visualIntel', label: 'Visual Intel', icon: Activity },
  { key: 'distributed', label: 'Distributed', icon: Cloud },
  { key: 'sdk', label: 'SDK', icon: FileSearch },
  { key: 'transparency', label: 'Transparency', icon: LockKeyhole },
  { key: 'premiumOs', label: 'Premium OS', icon: ShieldPlus },
  { key: 'aegisOs', label: 'Aegis OS', icon: CheckCircle2 },
  { key: 'operations', label: 'Ops Center', icon: Activity },
  { key: 'reliability', label: 'Reliability', icon: Gauge },
  { key: 'events', label: 'Events', icon: Database },
  { key: 'automation', label: 'Automation', icon: RefreshCw },
  { key: 'alerts', label: 'Alerts', icon: Bell },
  { key: 'profiles', label: 'Profiles', icon: Settings },
  { key: 'knowledge', label: 'Knowledge', icon: FileSearch },
  { key: 'assistant', label: 'Assistant', icon: Activity },
  { key: 'marketplace', label: 'Marketplace', icon: Cloud },
  { key: 'trust', label: 'Trust', icon: LockKeyhole },
  { key: 'brand', label: 'Identity', icon: ShieldPlus },
  { key: 'platform', label: 'Platform', icon: Layers3 },
  { key: 'vision', label: 'Vision', icon: CheckCircle2 },
  { key: 'modules', label: 'Modules', icon: Layers3 },
  { key: 'layers', label: 'Layers', icon: ShieldPlus },
  { key: 'scan', label: 'Scan', icon: Radar },
  { key: 'incidents', label: 'Incidents', icon: ClipboardCheck },
  { key: 'behavior', label: 'Behavior', icon: Activity },
  { key: 'processes', label: 'Processes', icon: Layers3 },
  { key: 'diagnostics', label: 'Diagnostics', icon: Gauge },
  { key: 'network', label: 'Network', icon: Cloud },
  { key: 'ransomware', label: 'Folders', icon: LockKeyhole },
  { key: 'sandbox', label: 'Sandbox', icon: LockKeyhole },
  { key: 'ai', label: 'AI Explain', icon: Activity },
  { key: 'plugins', label: 'Plugins', icon: Settings },
  { key: 'tools', label: 'Power Tools', icon: FileSearch },
  { key: 'score', label: 'Score', icon: Gauge },
  { key: 'ecosystem', label: 'Ecosystem', icon: Cloud },
  { key: 'quarantine', label: 'Quarantine', icon: ArchiveRestore },
  { key: 'intel', label: 'Intel', icon: Activity },
  { key: 'enterprise', label: 'Enterprise', icon: Building2 },
  { key: 'release', label: 'Release', icon: ShieldPlus },
  { key: 'updates', label: 'Updates', icon: RefreshCw },
  { key: 'logs', label: 'Logs', icon: History },
  { key: 'settings', label: 'Settings', icon: Settings }
];

export const securityCenterCards = [
  { label: 'Protection Score', value: '96', detail: 'All core shields active', tone: 'good', icon: Gauge },
  { label: 'Active Shields', value: '7', detail: 'Realtime, behavior, ransomware, updates', tone: 'good', icon: ShieldCheck },
  { label: 'Last Scan', value: 'Clean', detail: 'Quick scan, 18,431 files', tone: 'steady', icon: FileSearch },
  { label: 'Threats Blocked', value: '2', detail: '1 quarantined, 1 user reviewed', tone: 'warn', icon: ShieldAlert },
  { label: 'Ransomware', value: 'Guarded', detail: '4 protected folders', tone: 'good', icon: LockKeyhole },
  { label: 'Updates', value: 'Verified', detail: 'Signed manifest required', tone: 'steady', icon: RefreshCw },
  { label: 'Device Health', value: 'Good', detail: 'No corrupted components', tone: 'good', icon: Cpu },
  { label: 'Cloud Sync', value: 'Off', detail: 'Local-only privacy mode', tone: 'steady', icon: Cloud }
];

export const recommendedActions = [
  'Review high-risk process chain from Downloads',
  'Export incident report for invoice.pdf.exe',
  'Enable signed installer validation before beta release',
  'Run stress scan against 100k-file fixture set'
];

export const moduleCards = [
  { name: 'Aegis.Core', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Locked' },
  { name: 'Aegis.Scanner', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Ready' },
  { name: 'Aegis.Realtime', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Ready' },
  { name: 'Aegis.Behavior', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Ready' },
  { name: 'Aegis.Diagnostics', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Ready' },
  { name: 'Aegis.Firewall', version: '0.1.0', status: 'Visibility', api: '2026.05', hotReload: 'Ready' },
  { name: 'Aegis.Engine', version: '0.1.0', status: 'Loaded', api: '2026.05', hotReload: 'Locked' }
];

export const protectionLayers = [
  { layer: 'Layer 1', name: 'File Detection', status: 'Active', score: 98 },
  { layer: 'Layer 2', name: 'Heuristic Analysis', status: 'Active', score: 94 },
  { layer: 'Layer 3', name: 'Behavioral Monitoring', status: 'Active', score: 92 },
  { layer: 'Layer 4', name: 'Process Monitoring', status: 'Active', score: 91 },
  { layer: 'Layer 5', name: 'Script Monitoring', status: 'Active', score: 90 },
  { layer: 'Layer 6', name: 'Ransomware Protection', status: 'Active', score: 96 },
  { layer: 'Layer 7', name: 'Network Visibility', status: 'Visibility', score: 86 },
  { layer: 'Layer 8', name: 'System Integrity Checks', status: 'Active', score: 93 },
  { layer: 'Layer 9', name: 'User Awareness', status: 'Active', score: 97 }
];

export const diagnosticsMetrics = [
  { label: 'CPU', value: '18%', detail: 'Balanced policy' },
  { label: 'Memory', value: '42%', detail: 'Normal pressure' },
  { label: 'Disk', value: 'Good', detail: '71% free on C:' },
  { label: 'Network', value: '214 KB/s', detail: '7 open connections' },
  { label: 'Startup', value: '3', detail: '1 needs review' },
  { label: 'Services', value: '126', detail: '4 delayed start' }
];

export const diagnosticsRows = [
  { name: 'Startup Analyzer', status: '1 unknown entry', risk: 'Medium' },
  { name: 'Installed Applications', status: '84 apps indexed', risk: 'Low' },
  { name: 'Running Services', status: '126 services', risk: 'Low' },
  { name: 'Scheduled Tasks', status: '42 tasks', risk: 'Low' },
  { name: 'Driver List', status: 'All signed in sample', risk: 'Low' }
];

export const networkConnections = [
  { process: 'browser.exe', remote: 'updates.example.com:443', state: 'Established', bandwidth: '96 KB/s' },
  { process: 'aegis-agent.exe', remote: '127.0.0.1:pipe', state: 'Local IPC', bandwidth: '4 KB/s' },
  { process: 'unknown-tool.exe', remote: '203.0.113.24:443', state: 'Ask mode', bandwidth: '0 KB/s' }
];

export const firewallActions = [
  'Block connection',
  'Alert mode',
  'Ask before outbound for unknown apps',
  'Temporary network restrictions'
];

export const sandboxFacts = [
  { label: 'Filesystem', value: 'Limited' },
  { label: 'Network', value: 'Blocked' },
  { label: 'Runtime', value: '90 sec' },
  { label: 'Cleanup', value: 'Destroy after run' }
];

export const aiExplanations = [
  'This executable attempted persistence by creating a startup entry.',
  'This script launched multiple command interpreters rapidly.',
  'This process modified protected folders unusually fast.'
];

export const pluginCards = [
  { name: 'Network Plugin', permission: 'Read network metadata', status: 'Sandboxed' },
  { name: 'Diagnostics Plugin', permission: 'Read diagnostics', status: 'Sandboxed' },
  { name: 'Backup Plugin', permission: 'Write reports', status: 'Disabled' },
  { name: 'AI Analysis Plugin', permission: 'Read events', status: 'Sandboxed' }
];

export const powerTools = [
  'Process explorer',
  'File inspector',
  'Hash generator',
  'Signature tester',
  'YARA testing tool',
  'Log explorer',
  'Event inspector',
  'Scan debugger',
  'Harmless threat replay simulator'
];

export const scoreBreakdown = [
  { category: 'Protection', score: 96, deduction: 'No deduction' },
  { category: 'Updates', score: 90, deduction: 'Beta channel restart pending' },
  { category: 'System Health', score: 92, deduction: 'Disk scan pending' },
  { category: 'Startup Risk', score: 86, deduction: 'One unknown startup entry' },
  { category: 'Network Risk', score: 94, deduction: 'One unknown outbound app' },
  { category: 'User Configuration', score: 90, deduction: 'Cloud sync disabled' },
  { category: 'Threat History', score: 96, deduction: 'Recent high-risk event contained' }
];

export const ecosystemIntegrations = [
  { product: 'Aegis Diagnostics', status: 'Local module ready', enabled: true },
  { product: 'Aegis Firewall', status: 'Visibility mode', enabled: true },
  { product: 'Aegis VPN', status: 'Planned', enabled: false },
  { product: 'Aegis Backup', status: 'Planned', enabled: false },
  { product: 'Aegis Cloud', status: 'Optional sync', enabled: false },
  { product: 'Aegis AI Assistant', status: 'Explain-only mode', enabled: true },
  { product: 'Aegis Identity Protection', status: 'Roadmap', enabled: false }
];

export const healthCards = [
  { label: 'Protection', value: 'Active', detail: '4 engines online', tone: 'good', icon: ShieldCheck },
  { label: 'Threats', value: '2', detail: '1 high risk', tone: 'warn', icon: ShieldAlert },
  { label: 'Quarantine', value: '7', detail: '31 day cleanup', tone: 'steady', icon: ArchiveRestore },
  { label: 'Signatures', value: '2026.05.18.1', detail: 'stable channel', tone: 'steady', icon: Database }
];

export const scanModes = [
  { name: 'Quick', scope: 'Startup, Downloads, Temp, Desktop', eta: '4 min', active: true },
  { name: 'Full', scope: 'Drives, users, Program Files, ProgramData', eta: '48 min', active: false },
  { name: 'Deep', scope: 'Full plus archives, scripts, metadata, processes', eta: '91 min', active: false }
];

export const liveEvents = [
  { time: '12:48', level: 'high', text: 'Incident report generated for invoice.pdf.exe' },
  { time: '12:42', level: 'medium', text: 'Startup folder write staged for review' },
  { time: '12:36', level: 'low', text: 'Hash cache skipped 412 unchanged files' },
  { time: '12:31', level: 'high', text: 'Ransomware monitor reported rename burst' },
  { time: '12:22', level: 'info', text: 'Signature manifest verified' }
];

export const behaviorRules = [
  { rule: 'BEH-MANY-FILES-MODIFIED', score: 55, action: 'Confirm', status: 'Watching' },
  { rule: 'BEH-SCRIPT-SPAWNED-SHELL', score: 45, action: 'Notify', status: 'Ready' },
  { rule: 'BEH-TEMP-EXECUTABLE-DROP', score: 30, action: 'Notify', status: 'Ready' },
  { rule: 'BEH-REPEATED-FAILED-ACCESS', score: 25, action: 'Log', status: 'Ready' }
];

export const processTree = {
  name: 'explorer.exe',
  pid: 4128,
  score: 2,
  children: [
    {
      name: 'browser.exe',
      pid: 8840,
      score: 8,
      children: [
        {
          name: 'downloaded_file.exe',
          pid: 11320,
          score: 58,
          children: [{ name: 'powershell.exe', pid: 12244, score: 72, children: [] }]
        }
      ]
    }
  ]
};

export const quarantineItems = [
  {
    name: 'invoice.pdf.exe',
    detection: 'Suspicious.File.Heuristic.65',
    hash: '91b6...a22f',
    date: '2026-05-18 12:07',
    action: 'Restore blocked'
  },
  {
    name: 'eicar.com.txt',
    detection: 'Test.EICAR.Signature',
    hash: '275a...fd0f',
    date: '2026-05-18 10:13',
    action: 'Restore allowed'
  }
];

export const incidentSummary = {
  id: 'INC-2026-05-18-0007',
  detection: 'Suspicious.File.Heuristic.65',
  severity: 'High',
  path: 'C:/Users/User/Downloads/invoice.pdf.exe',
  sha256: '91b6f03c...a22f',
  action: 'Quarantined pending user confirmation',
  nextSteps: ['Inspect process chain', 'Submit false positive if trusted', 'Keep quarantine until verified']
};

export const incidentTimeline = [
  { time: '10:01', event: 'File downloaded', detail: 'Browser wrote invoice.pdf.exe to Downloads' },
  { time: '10:02', event: 'File scanned', detail: 'Double extension and executable header mismatch' },
  { time: '10:02', event: 'Behavior detected', detail: 'Downloaded process attempted shell launch' },
  { time: '10:03', event: 'File quarantined', detail: 'Restore requires confirmation' },
  { time: '10:04', event: 'User notified', detail: 'Incident report available as JSON, PDF, and HTML' }
];

export const updateHistory = [
  { version: '2026.05.18.1', channel: 'Stable', status: 'Verified', icon: RefreshCw },
  { version: '2026.05.17.4', channel: 'Stable', status: 'Applied', icon: Database },
  { version: '2026.05.16.2', channel: 'Stable', status: 'Rolled forward', icon: Gauge }
];

export const settingsProfiles = [
  { name: 'Balanced', detail: 'Notify before disruptive actions', icon: Bell },
  { name: 'Strict', detail: 'Auto-quarantine critical detections', icon: Siren },
  { name: 'Quiet', detail: 'Lower priority scans on battery', icon: Cpu },
  { name: 'Analyst', detail: 'Verbose event capture', icon: FileSearch },
  { name: 'Enterprise Managed', detail: 'Policy controlled by admin console', icon: Building2 }
];

export const threatIntel = [
  { label: 'Signature Version', value: '2026.05.18.1', detail: 'Stable channel' },
  { label: 'Rule Pack', value: 'windows-rules.4', detail: 'YARA placeholder pack' },
  { label: 'Top Category', value: 'Scripts', detail: '4 suspicious events' },
  { label: 'Common Location', value: 'Downloads', detail: '62% of local detections' },
  { label: 'Blocked Event', value: 'Child Shell', detail: '3 blocked or confirmed events' },
  { label: 'Changelog', value: '8 updates', detail: 'Hash and heuristic tuning' }
];

export const enterpriseDevices = [
  { name: 'GABRI-WORKSTATION', health: 96, policy: 'Balanced', signatures: '2026.05.18.1', threats: 2 },
  { name: 'DESIGN-LAPTOP', health: 82, policy: 'Performance', signatures: '2026.05.18.1', threats: 0 },
  { name: 'FINANCE-01', health: 74, policy: 'Strict', signatures: '2026.05.17.4', threats: 5 }
];

export const releaseChecklist = [
  { item: 'Rust tests and clippy', status: 'Passing' },
  { item: 'UI production build', status: 'Passing' },
  { item: 'Electron audit', status: 'Passing' },
  { item: 'Signed installer', status: 'Pending keys' },
  { item: 'Crash reporting', status: 'Planned' },
  { item: 'Rollback validation', status: 'Manifest ready' }
];

export const operationsCards = [
  { label: 'Active Incidents', value: '1', detail: 'High-risk chain contained', tone: 'warn', icon: ShieldAlert },
  { label: 'Protection Layers', value: '9/9', detail: 'All visible and reporting', tone: 'good', icon: Layers3 },
  { label: 'Automation Events', value: '14', detail: 'No destructive action without policy', tone: 'steady', icon: RefreshCw },
  { label: 'Ecosystem Health', value: 'Stable', detail: 'Core products aligned', tone: 'good', icon: Cloud }
];

export const reliabilityCards = [
  { name: 'Crash Recovery', status: 'Ready', detail: 'Capture dump, restart service, create report' },
  { name: 'Watchdog Monitor', status: 'Armed', detail: 'Heartbeat recovery for agent and modules' },
  { name: 'Safe Mode Startup', status: 'Available', detail: 'Minimal shields plus rollback and repair' },
  { name: 'Corruption Recovery', status: 'Guarded', detail: 'Verify signatures, config, quarantine metadata' },
  { name: 'Update Rollback', status: 'Prepared', detail: 'Return to last verified module snapshot' },
  { name: 'Stress Testing', status: 'Planned', detail: '100k-file, service crash, offline update fixtures' }
];

export const eventBusRows = [
  { event: 'ThreatDetected', source: 'Aegis.Scanner', severity: 'High', route: 'Logs, UI, incident, automation' },
  { event: 'ScanStarted', source: 'Aegis.Scanner', severity: 'Info', route: 'UI, analytics, enterprise' },
  { event: 'BehaviorTriggered', source: 'Aegis.Behavior', severity: 'Medium', route: 'Notification, automation, timeline' },
  { event: 'ConnectionBlocked', source: 'Aegis.Firewall', severity: 'Medium', route: 'Logs, network view, digest' },
  { event: 'FileQuarantined', source: 'Aegis.Quarantine', severity: 'High', route: 'Incident, timeline, alert' },
  { event: 'UpdateInstalled', source: 'Aegis.Updater', severity: 'Info', route: 'Update history, release reporting' }
];

export const automationWorkflows = [
  { name: 'Ransomware Score Over 80', trigger: 'BehaviorTriggered + ransomware_score > 80', response: 'Confirm isolate, quarantine, report, notify' },
  { name: 'Unknown Executable', trigger: 'ThreatDetected + unknown executable', response: 'Metadata reputation, prompt user' },
  { name: 'Update Failure', trigger: 'UpdateInstalled failed verification', response: 'Rollback, safe mode, reliability report' },
  { name: 'Critical Digest', trigger: 'Multiple high alerts in 10 minutes', response: 'Group alerts into incident summary' }
];

export const notificationRows = [
  { mode: 'Normal', behavior: 'Important alerts appear immediately', digest: 'Daily security summary' },
  { mode: 'Silent', behavior: 'Only critical alerts interrupt', digest: 'Deferred notification center' },
  { mode: 'Gaming', behavior: 'Critical only, low CPU scan policy', digest: 'Session summary after exit' },
  { mode: 'Enterprise', behavior: 'Managed alert routing', digest: 'Fleet and device reporting' }
];

export const advancedProfiles = [
  { name: 'Home User', ui: 'Standard', depth: 'Balanced scans', notifications: 'Important only' },
  { name: 'Power User', ui: 'Advanced', depth: 'Deep diagnostics', notifications: 'Detailed evidence' },
  { name: 'Developer', ui: 'Advanced', depth: 'Developer-aware exclusions', notifications: 'Low noise' },
  { name: 'Enterprise', ui: 'Managed', depth: 'Policy controlled', notifications: 'Admin routed' },
  { name: 'Silent Mode', ui: 'Simple', depth: 'Light scans', notifications: 'Critical only' },
  { name: 'Gaming Mode', ui: 'Simple', depth: 'Performance first', notifications: 'Critical only' }
];

export const knowledgeEntries = [
  { detection: 'Test.EICAR.Signature', severity: 'High test', risk: 'Safe validation file', remediation: 'Quarantine or delete test file' },
  { detection: 'Suspicious.File.Heuristic.65', severity: 'High', risk: 'Double extension and shell launch', remediation: 'Keep quarantined until trusted' },
  { detection: 'Behavior.Ransomware.RenameBurst', severity: 'Critical', risk: 'Mass rename pattern in protected folders', remediation: 'Stop related process and review report' },
  { detection: 'Network.UnknownOutbound', severity: 'Medium', risk: 'Unknown app requested outbound access', remediation: 'Allow once, block, or mark trusted' }
];

export const assistantWorkflows = [
  { task: 'Explain detection', sample: 'This file matched a test signature and is safe for validation.' },
  { task: 'Summarize incident', sample: 'A downloaded executable launched a shell after scan evidence increased risk.' },
  { task: 'Explain settings', sample: 'Strict mode raises sensitivity and may create more review prompts.' },
  { task: 'Guide troubleshooting', sample: 'Start with service health, update status, and component integrity.' }
];

export const marketplaceListings = [
  { name: 'Network Tools', publisher: 'Aegis Labs', trust: 'Signed manifest', permissions: 'Read network metadata' },
  { name: 'Developer Diagnostics', publisher: 'Aegis Labs', trust: 'Sandboxed', permissions: 'Read process and log metadata' },
  { name: 'Backup Connector', publisher: 'Aegis Labs', trust: 'Disabled preview', permissions: 'Write backup reports' },
  { name: 'Visualization Pack', publisher: 'Aegis Labs', trust: 'Signed manifest', permissions: 'Read event summaries' }
];

export const trustControls = [
  { control: 'Transparent logging', state: 'Enabled', detail: 'Every privileged action produces a local event' },
  { control: 'Clear permissions', state: 'Enabled', detail: 'Modules and plugins declare access explicitly' },
  { control: 'Explainable detections', state: 'Enabled', detail: 'Heuristic reasons and confidence are visible' },
  { control: 'False positive controls', state: 'Enabled', detail: 'Trust, restore, exclude, and user notes' },
  { control: 'Privacy controls', state: 'Enabled', detail: 'Cloud sync is optional and metadata-only by default' },
  { control: 'Easy uninstall', state: 'Required', detail: 'No hidden persistence or evasive self-protection' }
];

export const brandPillars = [
  { name: 'Industrial', detail: 'Command surfaces, precise spacing, restrained motion' },
  { name: 'Technical', detail: 'Evidence-first views and inspectable decisions' },
  { name: 'Premium', detail: 'Dark navy, silver separators, electric blue status light' },
  { name: 'Confident', detail: 'Calm language, no panic copy, user-controlled actions' }
];

export const platformLayers = [
  { layer: 'Platform Layer', status: 'Windows first', note: 'Filesystem, process, service, and notification adapters' },
  { layer: 'Core Engine', status: 'Portable', note: 'Scanner, signatures, events, policies, reports' },
  { layer: 'UI Layer', status: 'Cross-platform ready', note: 'Electron shell with platform integrations isolated' },
  { layer: 'Driver Layer', status: 'Planned', note: 'Separate signed implementations per OS later' },
  { layer: 'Update Layer', status: 'Portable', note: 'Signed packages, rollback, channels, offline updates' },
  { layer: 'Plugin Layer', status: 'Contracted', note: 'Sandbox, permissions, versioning, signatures' }
];

export const visionItems = [
  'Endpoint protection with visible, layered defenses',
  'Diagnostics and performance intelligence in one environment',
  'AI assistance that explains instead of blindly blocking',
  'A controlled plugin marketplace with signed modules',
  'Optional cloud and enterprise operations without uploading files by default',
  'Windows first, with core contracts ready for future Linux and macOS adapters'
];

export const aegisCoreCards = [
  { label: 'Module Control', value: '14', detail: 'Loaded through AegisCore', tone: 'good', icon: Layers3 },
  { label: 'Event Routes', value: '6', detail: 'UI, logs, automation, analytics, alerts, admin', tone: 'steady', icon: Database },
  { label: 'Permissions', value: 'Strict', detail: 'Every module declares access', tone: 'good', icon: LockKeyhole },
  { label: 'Service Mesh', value: 'Ready', detail: 'Agent, UI, updater, diagnostics coordinated', tone: 'steady', icon: Activity },
  { label: 'Plugin Host', value: 'Guarded', detail: 'Signed and sandboxed manifests only', tone: 'good', icon: Settings },
  { label: 'UI Sync', value: 'Live', detail: 'State mirrored from event bus', tone: 'steady', icon: RefreshCw }
];

export const systemGraphNodes = [
  { name: 'explorer.exe', kind: 'Process', risk: 2, x: 13, y: 52 },
  { name: 'browser.exe', kind: 'Process', risk: 8, x: 31, y: 37 },
  { name: 'downloaded_file.exe', kind: 'Process', risk: 58, x: 52, y: 48 },
  { name: 'powershell.exe', kind: 'Process', risk: 72, x: 72, y: 32 },
  { name: 'Aegis.Behavior', kind: 'Module', risk: 12, x: 63, y: 72 },
  { name: 'Incident INC-0007', kind: 'Threat', risk: 81, x: 84, y: 58 }
];

export const systemGraphEdges = [
  'explorer.exe -> browser.exe',
  'browser.exe -> downloaded_file.exe',
  'downloaded_file.exe -> powershell.exe',
  'powershell.exe -> Incident INC-0007',
  'Aegis.Behavior -> Incident INC-0007'
];

export const orchestrationPlans = [
  { trigger: 'Suspicious process launch', steps: 'Isolate pending review, capture metadata, scan related files, generate incident, notify user', review: 'User visible' },
  { trigger: 'Ransomware behavior', steps: 'Pause by policy, protect folders, snapshot affected files, launch recovery workflow', review: 'Policy gated' },
  { trigger: 'Network anomaly', steps: 'Capture metadata, correlate process, create incident, ask user', review: 'Prompt first' },
  { trigger: 'Service failure', steps: 'Capture dump, restart service, emit recovery event', review: 'Logged recovery' }
];

export const predictiveRiskSignals = [
  { category: 'Risky Apps', score: 14, trend: 'Stable', detail: 'One unknown executable from Downloads' },
  { category: 'Vulnerable Software', score: 22, trend: 'Rising', detail: 'Two apps need update validation' },
  { category: 'Startup Hygiene', score: 18, trend: 'Rising', detail: 'Unknown startup entry created this week' },
  { category: 'Network Anomalies', score: 9, trend: 'Stable', detail: 'One outbound ask-mode connection' },
  { category: 'Repeated Incidents', score: 27, trend: 'Rising', detail: 'Three suspicious script launches' }
];

export const aiOpsCapabilities = [
  { capability: 'Explain incidents', example: 'PowerShell launched from a temporary directory shortly after a downloaded executable executed.' },
  { capability: 'Summarize logs', example: 'Aegis grouped 42 events into one reviewable incident.' },
  { capability: 'Recommend actions', example: 'Keep the file quarantined until reputation and signature status are confirmed.' },
  { capability: 'Explain processes', example: 'This child process chain is unusual because a browser-launched executable spawned a shell.' },
  { capability: 'Device posture', example: 'Weak startup hygiene is increasing the local risk trend.' }
];

export const commandWidgets = [
  { name: 'Live System Map', state: 'Streaming', detail: 'Processes, services, drivers, connections, threats' },
  { name: 'AI Side Panel', state: 'Docked', detail: 'Explains current selection and recommends next steps' },
  { name: 'Detachable Windows', state: 'Designed', detail: 'Analyst panels can move into separate workspaces later' },
  { name: 'Advanced Filters', state: 'Active', detail: 'Risk, module, event type, time, relationship depth' },
  { name: 'Timeline Replay', state: 'Ready', detail: 'Reconstruct incident sequences from event history' },
  { name: 'Custom Dashboards', state: 'Planned', detail: 'Saved layouts per workspace and profile' }
];

export const syncMatrix = [
  { product: 'Aegis Identity', shared: 'Account, license, device identity', mode: 'Optional' },
  { product: 'Aegis Cloud', shared: 'Policy summaries and update state', mode: 'Metadata only' },
  { product: 'Aegis Sync', shared: 'Dashboards, settings, automation context', mode: 'User controlled' },
  { product: 'Aegis Core', shared: 'Local event and permission state', mode: 'Local first' }
];

export const observabilityRows = [
  { area: 'Performance Traces', value: '18', status: 'Healthy', source: 'Aegis.Diagnostics' },
  { area: 'Memory Analysis', value: '+32 MB', status: 'Watch', source: 'Aegis.Core' },
  { area: 'Disk Analysis', value: '71% free', status: 'Healthy', source: 'Aegis.Diagnostics' },
  { area: 'Service Dependencies', value: '12 mapped', status: 'Healthy', source: 'Aegis.Agent' },
  { area: 'Startup Timing', value: '2.8 sec', status: 'Healthy', source: 'Aegis.Diagnostics' },
  { area: 'Crash Analysis', value: '0 recent', status: 'Healthy', source: 'Aegis.Reliability' }
];

export const workspaceCards = [
  { name: 'Security Workspace', layout: 'Command center', widgets: 'Risk, incidents, event stream', automation: 'Visible' },
  { name: 'Diagnostics Workspace', layout: 'Observability grid', widgets: 'CPU, memory, disk, services', automation: 'Hidden' },
  { name: 'Developer Workspace', layout: 'Toolbench', widgets: 'Logs, hashing, signature tests', automation: 'Visible' },
  { name: 'Automation Workspace', layout: 'Workflow board', widgets: 'Runs, confirmations, policy preview', automation: 'Full' },
  { name: 'Network Workspace', layout: 'Topology', widgets: 'Connections, DNS, bandwidth', automation: 'Visible' },
  { name: 'Enterprise Workspace', layout: 'Fleet', widgets: 'Devices, policies, licenses', automation: 'Managed' }
];

export const visualIntelligenceSurfaces = [
  { surface: 'Threat Heatmap', live: 'Live', source: 'ThreatDetected events' },
  { surface: 'Risk Pulse', live: 'Animated', source: 'Predictive risk engine' },
  { surface: 'Integrity Ring', live: 'Live', source: 'Self-protection checks' },
  { surface: 'Network Topology', live: 'Streaming', source: 'Connection visibility' },
  { surface: 'Timeline Replay', live: 'Replay', source: 'Unified event bus' },
  { surface: 'Device Health', live: 'Live', source: 'Observability layer' }
];

export const distributedCapabilities = [
  { mode: 'Local-first operation', status: 'Primary', detail: 'Protection and intelligence work offline' },
  { mode: 'Optional cloud sync', status: 'Opt-in', detail: 'Metadata and settings only by default' },
  { mode: 'Enterprise fleet', status: 'Planned', detail: 'Policy, device health, remote scan requests' },
  { mode: 'Edge processing', status: 'Designed', detail: 'Analyze locally before summaries leave device' },
  { mode: 'Modular deployments', status: 'Ready', detail: 'Feature modules can be enabled independently' },
  { mode: 'Offline mode', status: 'Supported', detail: 'Offline updates and local reports remain available' }
];

export const developerApis = [
  { api: 'Aegis SDK', version: '0.1', permissions: 'Core models, schemas, reports', stability: 'Preview' },
  { api: 'Plugin API', version: '0.1', permissions: 'Signed manifests, sandbox permissions', stability: 'Preview' },
  { api: 'Automation API', version: '0.1', permissions: 'Workflow triggers and safe actions', stability: 'Preview' },
  { api: 'Dashboard API', version: '0.1', permissions: 'Read-only widgets and event summaries', stability: 'Preview' }
];

export const transparencyControls = [
  { name: 'Explainable detections', enabled: 'On', detail: 'Every detection exposes matched rules and reasons' },
  { name: 'Privacy-first design', enabled: 'On', detail: 'Personal files are not uploaded by default' },
  { name: 'Clear permissions', enabled: 'On', detail: 'Modules, plugins, and sync declare access' },
  { name: 'Visible automation', enabled: 'On', detail: 'Workflow steps are reviewable and logged' },
  { name: 'Optional telemetry', enabled: 'Off', detail: 'Telemetry stays user-controlled' },
  { name: 'Easy disable controls', enabled: 'Required', detail: 'Features can be paused without hidden behavior' }
];

export const premiumOsPillars = [
  { area: 'Installer', status: 'Planned', detail: 'Signed, explicit permissions, easy uninstall' },
  { area: 'Onboarding', status: 'Planned', detail: 'Fast setup with transparent protection choices' },
  { area: 'Animations', status: 'Active', detail: 'Subtle motion for live intelligence surfaces' },
  { area: 'Sounds', status: 'Concept', detail: 'Minimal critical-only audio language' },
  { area: 'Dashboards', status: 'Active', detail: 'Command surfaces with cinematic depth' },
  { area: 'Documentation', status: 'Active', detail: 'Architecture-first and privacy-clear' }
];

export const aegisOsVision = [
  'AegisCore becomes the local control plane for every module and workspace',
  'The live system graph turns machine activity into an intelligence map',
  'Automation becomes transparent orchestration, not hidden remote control',
  'Predictive risk turns repeated patterns into practical recommendations',
  'AI operations explains incidents, logs, processes, health, and posture',
  'Aegis remains local-first, privacy-first, and cross-platform ready'
];
