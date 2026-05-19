import {
  Activity,
  ArchiveRestore,
  Check,
  CircleStop,
  Cpu,
  Database,
  Download,
  FileSearch,
  FileText,
  Fingerprint,
  FolderOpen,
  Gauge,
  Globe2,
  LockKeyhole,
  Network,
  Play,
  Power,
  RotateCw,
  Search,
  Settings,
  ShieldAlert,
  ShieldCheck,
  ShieldOff,
  Siren,
  Trash2,
  Undo2,
  Wifi,
  X
} from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import { motion } from 'framer-motion';
import { useState } from 'react';
import type { ReactNode } from 'react';
import { GlassCard } from '../components/GlassCard';
import { MetricTile } from '../components/MetricTile';
import { PageHeader } from '../components/PageHeader';
import { StatusChip } from '../components/StatusChip';
import { ToggleSwitch } from '../components/ToggleSwitch';
import {
  connections,
  policyProfiles,
  privacyControls,
  protectedFolders,
  quarantineItems,
  threatEvents,
  threatRows,
  webEvents
} from '../data/operations';
import { useSecurityStore } from '../store/securityStore';
import type { ScanProfile, ScanResult, ScanSummary } from '../services/agent';

function Workspace({ children }: { children: ReactNode }) {
  return <motion.div initial={{ opacity: 0, y: 14 }} animate={{ opacity: 1, y: 0 }} className="mx-auto grid max-w-[1500px] grid-cols-12 gap-6">{children}</motion.div>;
}

function Panel({
  title,
  icon: Icon,
  children,
  className = ''
}: {
  title: string;
  icon: LucideIcon;
  children: ReactNode;
  className?: string;
}) {
  return (
    <GlassCard className={`p-6 ${className}`}>
      <div className="mb-5 flex items-center gap-3">
        <div className="grid h-10 w-10 place-items-center rounded-xl border border-white/[0.06] bg-white/[0.035]">
          <Icon className="h-5 w-5 text-aegis-green" />
        </div>
        <h2 className="text-[18px] font-semibold text-white">{title}</h2>
      </div>
      {children}
    </GlassCard>
  );
}

function ActionButton({
  children,
  onClick,
  tone = 'secondary',
  disabled = false
}: {
  children: ReactNode;
  onClick?: () => void;
  tone?: 'primary' | 'secondary' | 'danger';
  disabled?: boolean;
}) {
  const className = tone === 'primary' ? 'primary-button' : tone === 'danger' ? 'danger-button' : 'secondary-button';
  return (
    <button className={className} disabled={disabled} onClick={onClick}>
      {children}
    </button>
  );
}

function ScanSummaryCards({ summary }: { summary?: ScanSummary }) {
  return (
    <div className="grid grid-cols-2 gap-4 xl:grid-cols-4">
      <MetricTile icon={FileSearch} label="Files Scanned" value={String(summary?.files_scanned ?? 0)} detail="Completed by Rust scanner" tone="neutral" />
      <MetricTile icon={ShieldAlert} label="Threats" value={String(summary?.threats_found ?? 0)} detail="Malicious detections found" tone={(summary?.threats_found ?? 0) > 0 ? 'danger' : 'good'} />
      <MetricTile icon={Activity} label="Suspicious" value={String(summary?.suspicious_found ?? 0)} detail="Heuristic review candidates" tone={(summary?.suspicious_found ?? 0) > 0 ? 'warning' : 'neutral'} />
      <MetricTile icon={Gauge} label="Duration" value={`${summary?.duration_ms ?? 0}ms`} detail={`${summary?.files_skipped ?? 0} files skipped, ${summary?.errors ?? 0} errors`} tone="neutral" />
    </div>
  );
}

function findingsFromSummary(summary?: ScanSummary) {
  return (summary?.results ?? []).filter((result) => result.verdict !== 'clean');
}

export function SmartScanPage() {
  const scanProfile = useSecurityStore((state) => state.scanProfile);
  const setScanProfile = useSecurityStore((state) => state.setScanProfile);
  const customTarget = useSecurityStore((state) => state.customScanTarget);
  const setCustomTarget = useSecurityStore((state) => state.setCustomScanTarget);
  const browse = useSecurityStore((state) => state.browseForCustomScanTarget);
  const runScan = useSecurityStore((state) => state.runScan);
  const scanBusy = useSecurityStore((state) => state.scanBusy);
  const scanMessage = useSecurityStore((state) => state.scanMessage);
  const summary = useSecurityStore((state) => state.scanResponse?.summary);
  const profiles: Array<{ id: ScanProfile; title: string; detail: string; eta: string }> = [
    { id: 'quick', title: 'Quick Scan', detail: 'Startup, Downloads, Desktop, Documents, Temp', eta: 'Fast' },
    { id: 'full', title: 'Full Scan', detail: 'All resolved user and system targets', eta: 'Thorough' },
    { id: 'deep', title: 'Deep Scan', detail: 'Full profile with script and metadata review', eta: 'Detailed' },
    { id: 'custom', title: 'Custom Scan', detail: 'Choose one folder or file path', eta: 'Targeted' }
  ];

  return (
    <Workspace>
      <PageHeader label="Scanner Engine" title="Smart Scan" subtitle="Run the local Rust scanner from the desktop UI and review real JSON scan results." icon={Search} status={scanBusy ? 'Scanning' : 'Ready'} />

      <Panel title="Scan Command" icon={FileSearch} className="col-span-12 xl:col-span-7">
        <div className="grid gap-4 md:grid-cols-2">
          {profiles.map((profile) => (
            <button
              key={profile.id}
              onClick={() => setScanProfile(profile.id)}
              className={`rounded-2xl border p-5 text-left transition hover:-translate-y-0.5 ${
                scanProfile === profile.id ? 'border-aegis-green/35 bg-aegis-green/10' : 'border-white/[0.06] bg-white/[0.025]'
              }`}
            >
              <div className="flex items-center justify-between gap-3">
                <strong className="text-white">{profile.title}</strong>
                <StatusChip label={profile.eta} tone={scanProfile === profile.id ? 'good' : 'neutral'} />
              </div>
              <p className="mt-3 text-sm leading-6 text-white/55">{profile.detail}</p>
            </button>
          ))}
        </div>

        <div className="mt-5 grid gap-3 md:grid-cols-[minmax(0,1fr)_auto]">
          <input
            value={customTarget}
            onChange={(event) => setCustomTarget(event.target.value)}
            placeholder="Custom scan target, optional for quick/full/deep"
            className="h-12 rounded-2xl border border-white/[0.06] bg-black/20 px-4 text-sm text-white outline-none transition placeholder:text-white/30 focus:border-aegis-green/35"
          />
          <ActionButton onClick={() => void browse()}>
            <FolderOpen className="h-4 w-4" /> Browse
          </ActionButton>
        </div>

        <div className="mt-5 flex flex-wrap items-center gap-3">
          <ActionButton tone="primary" disabled={scanBusy} onClick={() => void runScan()}>
            <Play className="h-4 w-4" /> {scanBusy ? 'Scanning...' : 'Run Scan'}
          </ActionButton>
          <ActionButton disabled={scanBusy} onClick={() => void runScan('quick')}>
            <RotateCw className="h-4 w-4" /> Quick Scan Now
          </ActionButton>
          <span className="text-sm text-white/45">{scanMessage}</span>
        </div>
      </Panel>

      <Panel title="Current Result" icon={Gauge} className="col-span-12 xl:col-span-5">
        <ScanSummaryCards summary={summary} />
      </Panel>

      <Panel title="Findings" icon={ShieldAlert} className="col-span-12">
        <FindingsList findings={findingsFromSummary(summary)} />
      </Panel>
    </Workspace>
  );
}

export function RealTimeProtectionPage() {
  const status = useSecurityStore((state) => state.agentResponse?.status);
  const guardRunning = useSecurityStore((state) => state.agentResponse?.guardRunning);
  const message = useSecurityStore((state) => state.agentMessage);
  const busy = useSecurityStore((state) => state.agentBusy);
  const startGuard = useSecurityStore((state) => state.startGuard);
  const stopGuard = useSecurityStore((state) => state.stopGuard);
  const armGuard = useSecurityStore((state) => state.armGuard);
  const disarmGuard = useSecurityStore((state) => state.disarmGuard);
  const refresh = useSecurityStore((state) => state.refreshAgent);
  const armed = status?.armed !== false;

  return (
    <Workspace>
      <PageHeader label="Live Guard" title="Real-Time Protection" subtitle="Start, arm, disarm, and inspect the local guard from the GUI." icon={ShieldCheck} status={armed ? 'Armed' : 'Paused'} />

      <Panel title="Guard Control" icon={Power} className="col-span-12 xl:col-span-7">
        <div className="grid gap-4 md:grid-cols-4">
          <ActionButton tone="primary" disabled={busy} onClick={() => void startGuard()}><Play className="h-4 w-4" /> Start</ActionButton>
          <ActionButton disabled={busy} onClick={() => void armGuard()}><ShieldCheck className="h-4 w-4" /> Arm</ActionButton>
          <ActionButton disabled={busy} onClick={() => void disarmGuard()}><ShieldOff className="h-4 w-4" /> Disarm</ActionButton>
          <ActionButton tone="danger" disabled={busy} onClick={() => void stopGuard()}><CircleStop className="h-4 w-4" /> Stop</ActionButton>
        </div>
        <div className="mt-6 grid gap-4 md:grid-cols-3">
          <MetricTile icon={ShieldCheck} label="Armed" value={armed ? 'Yes' : 'No'} detail={message} tone={armed ? 'good' : 'warning'} />
          <MetricTile icon={Activity} label="Process" value={guardRunning ? 'Running' : 'Stopped'} detail="Foreground child process from GUI" tone={guardRunning ? 'good' : 'neutral'} />
          <MetricTile icon={RotateCw} label="Interval" value={`${status?.interval_seconds ?? 30}s`} detail="Periodic scan cadence" tone="neutral" />
        </div>
      </Panel>

      <Panel title="Watched Locations" icon={FolderOpen} className="col-span-12 xl:col-span-5">
        <div className="space-y-3">
          {(status?.watched_paths ?? []).map((path) => (
            <div key={path} className="flex items-center gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-3">
              <Check className="h-4 w-4 text-aegis-green" />
              <span className="min-w-0 truncate text-sm text-white/70">{path}</span>
            </div>
          ))}
          {!status?.watched_paths?.length && <p className="text-sm text-white/55">Start the guard to load default watch paths.</p>}
        </div>
        <button className="secondary-button mt-5 w-full" onClick={() => void refresh()}>
          <RotateCw className="h-4 w-4" /> Refresh Status
        </button>
      </Panel>
    </Workspace>
  );
}

export function ThreatCenterPage() {
  const findings = findingsFromSummary(useSecurityStore((state) => state.scanResponse?.summary));
  return (
    <Workspace>
      <PageHeader label="Detection Review" title="Threat Center" subtitle="Review detections, confidence, rule matches, hashes, and action recommendations." icon={ShieldAlert} status={`${findings.length || threatRows.length} items`} />
      <Panel title="Current Detections" icon={ShieldAlert} className="col-span-12 xl:col-span-8">
        {findings.length > 0 ? <FindingsList findings={findings} /> : (
          <div className="space-y-3">
            {threatRows.map((row) => (
              <div key={row.name} className="rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4">
                <div className="flex flex-wrap items-center justify-between gap-3">
                  <strong className="text-white">{row.name}</strong>
                  <StatusChip label={row.severity} tone={row.severity === 'High' ? 'warning' : 'neutral'} />
                </div>
                <div className="mt-3 text-sm text-white/52">{row.path}</div>
                <div className="mt-2 text-xs text-white/35">SHA256 {row.hash}</div>
                <div className="mt-4 flex flex-wrap gap-2">
                  {row.reasons.map((reason) => <StatusChip key={reason} label={reason} tone="neutral" />)}
                </div>
              </div>
            ))}
          </div>
        )}
      </Panel>
      <Panel title="Threat Timeline" icon={Activity} className="col-span-12 xl:col-span-4">
        <Timeline />
      </Panel>
    </Workspace>
  );
}

export function FirewallPage() {
  const [blocked, setBlocked] = useState<string[]>([]);
  return (
    <Workspace>
      <PageHeader label="Network Visibility" title="Firewall" subtitle="Inspect active outbound connections and apply safe user-controlled rules." icon={Network} status="Visibility mode" />
      <Panel title="Connections" icon={Wifi} className="col-span-12">
        <div className="grid gap-3">
          {connections.map((connection) => {
            const isBlocked = blocked.includes(connection.process);
            return (
              <div key={`${connection.process}-${connection.remote}`} className="grid gap-4 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4 xl:grid-cols-[1fr_1fr_90px_110px_180px] xl:items-center">
                <div><strong className="text-white">{connection.process}</strong><div className="mt-1 text-sm text-white/45">{connection.rule}</div></div>
                <div className="text-sm text-white/64">{connection.remote}:{connection.port}</div>
                <StatusChip label={connection.risk} tone={connection.risk === 'Review' ? 'warning' : 'good'} />
                <span className="text-sm text-white/45">{connection.bandwidth}</span>
                <ActionButton tone={isBlocked ? 'secondary' : 'danger'} onClick={() => setBlocked((items) => isBlocked ? items.filter((item) => item !== connection.process) : [...items, connection.process])}>
                  {isBlocked ? <Check className="h-4 w-4" /> : <X className="h-4 w-4" />} {isBlocked ? 'Unblock' : 'Block'}
                </ActionButton>
              </div>
            );
          })}
        </div>
      </Panel>
    </Workspace>
  );
}

export function WebProtectionPage() {
  return (
    <Workspace>
      <PageHeader label="Web Shield" title="Web Protection" subtitle="Track safe download checks, domain reputation, and blocked test events." icon={Globe2} status="Enabled" />
      <Panel title="Recent Web Events" icon={Globe2} className="col-span-12 xl:col-span-8">
        <div className="space-y-3">
          {webEvents.map((event) => (
            <div key={`${event.domain}-${event.time}`} className="grid gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4 md:grid-cols-[1fr_150px_120px_80px] md:items-center">
              <strong className="text-white">{event.domain}</strong>
              <span className="text-sm text-white/55">{event.category}</span>
              <StatusChip label={event.verdict} tone={event.verdict === 'Blocked' ? 'danger' : event.verdict === 'Warned' ? 'warning' : 'good'} />
              <span className="text-sm text-white/38">{event.time}</span>
            </div>
          ))}
        </div>
      </Panel>
      <Panel title="Protection Controls" icon={Settings} className="col-span-12 xl:col-span-4">
        {['Block dangerous downloads', 'Warn on new domains', 'Scan downloaded files', 'Use local reputation cache'].map((item) => (
          <div key={item} className="flex items-center justify-between border-b border-white/[0.06] py-4 last:border-b-0">
            <span className="text-sm text-white/68">{item}</span>
            <ToggleSwitch checked label={item} />
          </div>
        ))}
      </Panel>
    </Workspace>
  );
}

export function RansomwarePage() {
  return (
    <Workspace>
      <PageHeader label="Protected Folders" title="Ransomware Shield" subtitle="Watch protected folders for rapid renames, deletion bursts, and encryption-like writes." icon={LockKeyhole} status="Protected" />
      <Panel title="Folders" icon={FolderOpen} className="col-span-12 xl:col-span-7">
        <div className="grid gap-3">
          {protectedFolders.map((folder) => (
            <div key={folder.folder} className="grid grid-cols-[1fr_90px_130px] items-center rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4">
              <strong className="text-white">{folder.folder}</strong>
              <span className="text-sm text-white/45">{folder.events} events</span>
              <StatusChip label={folder.state} tone="good" />
            </div>
          ))}
        </div>
      </Panel>
      <Panel title="Behavior Rules" icon={Siren} className="col-span-12 xl:col-span-5">
        {['Rapid file renames', 'Mass extension changes', 'Protected-folder deletion burst', 'Suspicious process writing many files'].map((rule) => (
          <div key={rule} className="flex items-center justify-between border-b border-white/[0.06] py-4 last:border-b-0">
            <span className="text-sm text-white/68">{rule}</span>
            <StatusChip label="Watching" tone="good" />
          </div>
        ))}
      </Panel>
    </Workspace>
  );
}

export function PrivacyPage() {
  return (
    <Workspace>
      <PageHeader label="Trust Controls" title="Privacy" subtitle="Keep reporting local-first, transparent, and user controlled." icon={Fingerprint} status="Local-first" />
      <Panel title="Data Controls" icon={Fingerprint} className="col-span-12 xl:col-span-7">
        <div className="space-y-3">
          {privacyControls.map((control) => (
            <div key={control.name} className="grid gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4 md:grid-cols-[1fr_120px_auto] md:items-center">
              <div><strong className="text-white">{control.name}</strong><div className="mt-1 text-sm text-white/45">{control.detail}</div></div>
              <StatusChip label={control.state} tone={control.state === 'Off' || control.state === 'Blocked' ? 'good' : 'neutral'} />
              <ToggleSwitch checked={control.state !== 'Off'} label={control.name} />
            </div>
          ))}
        </div>
      </Panel>
      <Panel title="Transparency" icon={FileText} className="col-span-12 xl:col-span-5">
        <p className="text-sm leading-7 text-white/58">Aegis does not upload personal files by default. Security actions stay visible in local logs, and automation remains reviewable.</p>
        <div className="mt-6 grid gap-3">
          <ActionButton><FileText className="h-4 w-4" /> View Privacy Notes</ActionButton>
          <ActionButton><Download className="h-4 w-4" /> Export Local Settings</ActionButton>
        </div>
      </Panel>
    </Workspace>
  );
}

export function QuarantinePage() {
  const [items, setItems] = useState(quarantineItems);
  return (
    <Workspace>
      <PageHeader label="Containment Vault" title="Quarantine" subtitle="Review isolated files, restore safely, trust safe files, or remove records intentionally." icon={ArchiveRestore} status={`${items.length} items`} />
      <Panel title="Quarantined Files" icon={ArchiveRestore} className="col-span-12">
        <div className="grid gap-3">
          {items.map((item) => (
            <div key={item.id} className="grid gap-4 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4 xl:grid-cols-[1fr_1fr_150px_260px] xl:items-center">
              <div><strong className="text-white">{item.name}</strong><div className="mt-1 text-xs text-white/38">{item.id}</div></div>
              <span className="text-sm text-white/62">{item.detection}</span>
              <StatusChip label={item.status} tone={item.status.includes('locked') ? 'warning' : 'good'} />
              <div className="flex flex-wrap gap-2">
                <ActionButton><Undo2 className="h-4 w-4" /> Restore</ActionButton>
                <ActionButton onClick={() => setItems((current) => current.filter((candidate) => candidate.id !== item.id))}><Trash2 className="h-4 w-4" /> Remove</ActionButton>
              </div>
            </div>
          ))}
        </div>
      </Panel>
    </Workspace>
  );
}

export function ReportsPage() {
  const summary = useSecurityStore((state) => state.scanResponse?.summary);

  const exportJson = () => {
    const report = {
      generated_at: new Date().toISOString(),
      product: 'Aegis AntiVirus',
      scan_summary: summary ?? null,
      timeline: threatEvents
    };
    const url = URL.createObjectURL(new Blob([JSON.stringify(report, null, 2)], { type: 'application/json' }));
    const anchor = document.createElement('a');
    anchor.href = url;
    anchor.download = `aegis-report-${Date.now()}.json`;
    anchor.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Workspace>
      <PageHeader label="Evidence" title="Reports" subtitle="Generate local incident and scan reports without uploading personal files." icon={FileText} status="Export ready" />
      <Panel title="Latest Scan Summary" icon={Database} className="col-span-12 xl:col-span-7">
        <ScanSummaryCards summary={summary} />
        <div className="mt-5 flex flex-wrap gap-3">
          <ActionButton tone="primary" onClick={exportJson}><Download className="h-4 w-4" /> Export JSON</ActionButton>
          <ActionButton><FileText className="h-4 w-4" /> HTML Preview</ActionButton>
          <ActionButton><FileText className="h-4 w-4" /> PDF Later</ActionButton>
        </div>
      </Panel>
      <Panel title="Incident Timeline" icon={Activity} className="col-span-12 xl:col-span-5">
        <Timeline />
      </Panel>
    </Workspace>
  );
}

export function SettingsPage() {
  const activePolicy = useSecurityStore((state) => state.activePolicy);
  const setActivePolicy = useSecurityStore((state) => state.setActivePolicy);
  return (
    <Workspace>
      <PageHeader label="Policy Profiles" title="Settings" subtitle="Tune scan depth, notification behavior, privacy, updates, and protection posture." icon={Settings} status={activePolicy} />
      <Panel title="Protection Policy" icon={Settings} className="col-span-12 xl:col-span-8">
        <div className="grid gap-4 md:grid-cols-2">
          {policyProfiles.map((profile) => (
            <button
              key={profile.name}
              onClick={() => setActivePolicy(profile.name)}
              className={`rounded-2xl border p-5 text-left transition hover:-translate-y-0.5 ${
                activePolicy === profile.name ? 'border-aegis-green/35 bg-aegis-green/10' : 'border-white/[0.06] bg-white/[0.025]'
              }`}
            >
              <div className="flex items-center justify-between">
                <strong className="text-white">{profile.name}</strong>
                {activePolicy === profile.name && <Check className="h-4 w-4 text-aegis-green" />}
              </div>
              <p className="mt-3 text-sm leading-6 text-white/55">{profile.detail}</p>
              <div className="mt-4 flex gap-2">
                <StatusChip label={`CPU ${profile.cpu}`} tone="neutral" />
                <StatusChip label={profile.notifications} tone="neutral" />
              </div>
            </button>
          ))}
        </div>
      </Panel>
      <Panel title="System Rules" icon={Cpu} className="col-span-12 xl:col-span-4">
        {['Battery saver mode', 'Skip unchanged files', 'Parallel scan limit', 'Signed update manifests', 'Low-noise alerts'].map((item) => (
          <div key={item} className="flex items-center justify-between border-b border-white/[0.06] py-4 last:border-b-0">
            <span className="text-sm text-white/68">{item}</span>
            <ToggleSwitch checked label={item} />
          </div>
        ))}
      </Panel>
    </Workspace>
  );
}

function FindingsList({ findings }: { findings: ScanResult[] }) {
  if (findings.length === 0) {
    return (
      <div className="grid min-h-[180px] place-items-center rounded-2xl border border-white/[0.06] bg-white/[0.025] p-6 text-center">
        <div>
          <ShieldCheck className="mx-auto h-9 w-9 text-aegis-green" />
          <div className="mt-4 font-semibold text-white">No findings in the latest scan</div>
          <p className="mt-2 text-sm text-white/48">Run a quick, full, deep, or custom scan to populate this table.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {findings.map((finding) => (
        <div key={`${finding.path}-${finding.verdict}`} className="rounded-2xl border border-white/[0.06] bg-white/[0.025] p-4">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <strong className="text-white">{finding.detection_name ?? finding.verdict}</strong>
            <StatusChip label={`${finding.confidence_score}% confidence`} tone={finding.verdict === 'malicious' ? 'danger' : 'warning'} />
          </div>
          <div className="mt-3 break-all text-sm text-white/58">{finding.path}</div>
          <div className="mt-2 break-all text-xs text-white/35">SHA256 {finding.sha256 ?? 'not available'}</div>
          {finding.heuristics.length > 0 && (
            <div className="mt-4 flex flex-wrap gap-2">
              {finding.heuristics.map((heuristic) => (
                <StatusChip key={heuristic.rule_id} label={`${heuristic.rule_id} +${heuristic.score}`} tone="warning" />
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

function Timeline() {
  return (
    <div className="space-y-3">
      {threatEvents.map((event) => (
        <div key={`${event.time}-${event.title}`} className="grid grid-cols-[54px_minmax(0,1fr)] gap-3 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-3">
          <span className="text-sm font-semibold text-aegis-green">{event.time}</span>
          <div>
            <div className="font-medium text-white">{event.title}</div>
            <div className="mt-1 text-sm text-white/45">{event.detail}</div>
          </div>
        </div>
      ))}
    </div>
  );
}
