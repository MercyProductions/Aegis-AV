import { useEffect, useMemo, useState } from 'react';
import type { CSSProperties } from 'react';
import {
  Activity,
  ArchiveRestore,
  Bell,
  Building2,
  CheckCircle2,
  CirclePause,
  Cloud,
  Database,
  FileJson,
  FileSearch,
  Gauge,
  Layers3,
  LockKeyhole,
  Radar,
  RefreshCw,
  Settings,
  ShieldCheck,
  ShieldAlert,
  ShieldPlus,
  Zap
} from 'lucide-react';
import {
  advancedProfiles,
  aegisCoreCards,
  aegisOsVision,
  aiExplanations,
  aiOpsCapabilities,
  assistantWorkflows,
  automationWorkflows,
  behaviorRules,
  brandPillars,
  commandWidgets,
  developerApis,
  diagnosticsMetrics,
  diagnosticsRows,
  distributedCapabilities,
  ecosystemIntegrations,
  enterpriseDevices,
  eventBusRows,
  firewallActions,
  healthCards,
  incidentSummary,
  incidentTimeline,
  knowledgeEntries,
  liveEvents,
  marketplaceListings,
  moduleCards,
  networkConnections,
  navItems,
  notificationRows,
  observabilityRows,
  operationsCards,
  orchestrationPlans,
  platformLayers,
  pluginCards,
  predictiveRiskSignals,
  premiumOsPillars,
  powerTools,
  processTree,
  protectionLayers,
  quarantineItems,
  recommendedActions,
  releaseChecklist,
  reliabilityCards,
  sandboxFacts,
  scanModes,
  scoreBreakdown,
  settingsProfiles,
  securityCenterCards,
  syncMatrix,
  systemGraphEdges,
  systemGraphNodes,
  threatIntel,
  transparencyControls,
  trustControls,
  updateHistory,
  visualIntelligenceSurfaces,
  visionItems,
  workspaceCards,
  type PageKey
} from './data/securityModel';

interface ProcessNodeUi {
  name: string;
  pid: number;
  score: number;
  children: ProcessNodeUi[];
}

interface AgentStatus {
  armed: boolean;
  interval_seconds: number;
  watched_paths: string[];
  last_scan_unix_seconds: number | null;
  last_scan_files: number;
  last_scan_suspicious: number;
  last_scan_threats: number;
  last_scan_errors: number;
  state_file?: string;
  log_file?: string;
}

interface AgentResponse {
  ok: boolean;
  stdout: string;
  stderr: string;
  error?: string;
  agentPath?: string;
  guardRunning?: boolean;
  status?: AgentStatus;
}

declare global {
  interface Window {
    aegis?: {
      version: string;
      agent?: {
        status: () => Promise<AgentResponse>;
        arm: () => Promise<AgentResponse>;
        disarm: () => Promise<AgentResponse>;
        start: () => Promise<AgentResponse>;
        stop: () => Promise<AgentResponse>;
      };
    };
  }
}

export function App() {
  const [page, setPage] = useState<PageKey>('security');
  const activeNav = useMemo(() => navItems.find((item) => item.key === page), [page]);

  return (
    <main className="shell">
      <aside className="sidebar" aria-label="Aegis navigation">
        <div className="brand">
          <div className="brandMark"><ShieldCheck size={26} /></div>
          <div>
            <strong>Aegis</strong>
            <span>Endpoint Protection</span>
          </div>
        </div>
        <nav className="navList">
          {navItems.map((item) => {
            const Icon = item.icon;
            return (
              <button
                key={item.key}
                className={item.key === page ? 'navItem active' : 'navItem'}
                onClick={() => setPage(item.key)}
                title={item.label}
              >
                <Icon size={18} />
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
        <div className="agentPanel">
          <span className="pulseDot" />
          <div>
            <strong>Agent Online</strong>
            <small>Named pipe ready</small>
          </div>
        </div>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <span className="eyebrow">{activeNav?.label}</span>
            <h1>{headlineFor(page)}</h1>
          </div>
          <div className="topActions">
            <button title="Pause protection"><CirclePause size={18} /></button>
            <button title="Run quick scan"><Radar size={18} /></button>
            <button title="Check updates"><RefreshCw size={18} /></button>
          </div>
        </header>
        {page === 'security' && <SecurityCenterPage />}
        {page === 'deviceControl' && <DeviceControlPage />}
        {page === 'aegisCore' && <AegisCorePage />}
        {page === 'systemGraph' && <SystemGraphPage />}
        {page === 'orchestration' && <OrchestrationPage />}
        {page === 'predictiveRisk' && <PredictiveRiskPage />}
        {page === 'aiOps' && <AiOpsPage />}
        {page === 'commandOs' && <CommandOsPage />}
        {page === 'sync' && <SyncPage />}
        {page === 'observability' && <ObservabilityPage />}
        {page === 'workspaces' && <WorkspacesPage />}
        {page === 'visualIntel' && <VisualIntelPage />}
        {page === 'distributed' && <DistributedPage />}
        {page === 'sdk' && <SdkPage />}
        {page === 'transparency' && <TransparencyPage />}
        {page === 'premiumOs' && <PremiumOsPage />}
        {page === 'aegisOs' && <AegisOsPage />}
        {page === 'operations' && <OperationsPage />}
        {page === 'reliability' && <ReliabilityPage />}
        {page === 'events' && <EventsPage />}
        {page === 'automation' && <AutomationPage />}
        {page === 'alerts' && <AlertsPage />}
        {page === 'profiles' && <ProfilesPage />}
        {page === 'knowledge' && <KnowledgePage />}
        {page === 'assistant' && <AssistantPage />}
        {page === 'marketplace' && <MarketplacePage />}
        {page === 'trust' && <TrustPage />}
        {page === 'brand' && <BrandPage />}
        {page === 'platform' && <PlatformPage />}
        {page === 'vision' && <VisionPage />}
        {page === 'modules' && <ModulesPage />}
        {page === 'layers' && <LayersPage />}
        {page === 'scan' && <ScanPage />}
        {page === 'incidents' && <IncidentsPage />}
        {page === 'behavior' && <BehaviorPage />}
        {page === 'processes' && <ProcessesPage />}
        {page === 'diagnostics' && <DiagnosticsPage />}
        {page === 'network' && <NetworkPage />}
        {page === 'ransomware' && <RansomwarePage />}
        {page === 'sandbox' && <SandboxPage />}
        {page === 'ai' && <AiPage />}
        {page === 'plugins' && <PluginsPage />}
        {page === 'tools' && <PowerToolsPage />}
        {page === 'score' && <ScorePage />}
        {page === 'ecosystem' && <EcosystemPage />}
        {page === 'quarantine' && <QuarantinePage />}
        {page === 'intel' && <ThreatIntelPage />}
        {page === 'enterprise' && <EnterprisePage />}
        {page === 'release' && <ReleasePage />}
        {page === 'updates' && <UpdatesPage />}
        {page === 'logs' && <LogsPage />}
        {page === 'settings' && <SettingsPage />}
      </section>
    </main>
  );
}

function DeviceControlPage() {
  const [response, setResponse] = useState<AgentResponse | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState('Ready');
  const api = window.aegis?.agent;
  const status = response?.status;

  const runAction = async (label: string, action: (() => Promise<AgentResponse>) | undefined) => {
    if (!action) {
      setMessage('Open Aegis in the Electron desktop app to control the guard agent.');
      return;
    }
    setBusy(true);
    setMessage(`${label}...`);
    try {
      const next = await action();
      setResponse(next);
      setMessage(next.ok ? `${label} complete.` : next.error ?? `${label} failed.`);
    } catch (error) {
      setMessage(error instanceof Error ? error.message : `${label} failed.`);
    } finally {
      setBusy(false);
    }
  };

  useEffect(() => {
    if (!api) {
      setMessage('Desktop agent controls are available in the Electron app.');
      return;
    }
    void runAction('Refreshing status', api.status);
  }, []);

  return (
    <div className="pageGrid securityGrid">
      <section className="commandHero">
        <div className={status?.armed ? 'statusRing large armedRing' : 'statusRing large'}>
          <span>{status?.armed ? 'ON' : 'OFF'}</span>
          <small>guard</small>
        </div>
        <div className="statusCopy">
          <h2>Device Guard Control</h2>
          <p>Start, arm, disarm, and stop the local Aegis guard from the desktop app. The guard remains visible and user-controlled.</p>
        </div>
        <button className="primaryAction" disabled={busy || !api} onClick={() => runAction('Start guard', api?.start)}>
          <ShieldPlus size={18} />Start Guard
        </button>
      </section>
      <section className="controlPanel">
        <button disabled={busy || !api} onClick={() => runAction('Arm guard', api?.arm)}>
          <ShieldCheck size={18} />Arm
        </button>
        <button disabled={busy || !api} onClick={() => runAction('Disarm guard', api?.disarm)}>
          <CirclePause size={18} />Disarm
        </button>
        <button disabled={busy || !api} onClick={() => runAction('Stop guard', api?.stop)}>
          <LockKeyhole size={18} />Stop Guard
        </button>
        <button disabled={busy || !api} onClick={() => runAction('Refresh status', api?.status)}>
          <RefreshCw size={18} />Refresh
        </button>
      </section>
      <section className="metricGrid securityMetrics">
        <article className={`metricCard ${status?.armed ? 'good' : 'warn'}`}>
          <ShieldCheck size={21} />
          <span>Armed</span>
          <strong>{status?.armed ? 'Yes' : 'No'}</strong>
          <small>{response?.guardRunning ? 'Guard process running' : 'Guard process stopped'}</small>
        </article>
        <article className="metricCard steady">
          <Gauge size={21} />
          <span>Interval</span>
          <strong>{status?.interval_seconds ?? 30}s</strong>
          <small>Periodic scan cadence</small>
        </article>
        <article className="metricCard steady">
          <FileSearch size={21} />
          <span>Last Scan</span>
          <strong>{status?.last_scan_files ?? 0}</strong>
          <small>{status?.last_scan_unix_seconds ? `Unix ${status.last_scan_unix_seconds}` : 'Never scanned'}</small>
        </article>
        <article className={`metricCard ${(status?.last_scan_threats ?? 0) > 0 ? 'warn' : 'good'}`}>
          <ShieldAlert size={21} />
          <span>Findings</span>
          <strong>{status?.last_scan_threats ?? 0}</strong>
          <small>{status?.last_scan_suspicious ?? 0} suspicious, {status?.last_scan_errors ?? 0} errors</small>
        </article>
      </section>
      <section className="actionPanel">
        <h2>Watched Locations</h2>
        {(status?.watched_paths ?? []).map((item) => (
          <div className="actionRow" key={item}>
            <CheckCircle2 size={16} />
            <span>{item}</span>
          </div>
        ))}
        {!status?.watched_paths?.length && (
          <div className="actionRow">
            <Activity size={16} />
            <span>{message}</span>
          </div>
        )}
      </section>
      <section className="detailDrawer">
        <h2>Agent Details</h2>
        <dl>
          <dt>Status</dt>
          <dd>{message}</dd>
          <dt>Agent</dt>
          <dd>{response?.agentPath ?? 'Build target/release/aegis-agent.exe first'}</dd>
          <dt>State</dt>
          <dd>{status?.state_file ?? 'Not loaded'}</dd>
          <dt>Logs</dt>
          <dd>{status?.log_file ?? 'Not loaded'}</dd>
        </dl>
      </section>
    </div>
  );
}

function AegisCorePage() {
  return (
    <div className="pageGrid securityGrid">
      <section className="commandHero">
        <div className="statusRing large">
          <span>98</span>
          <small>core</small>
        </div>
        <div className="statusCopy">
          <h2>AegisCore Runtime</h2>
          <p>The central control plane coordinates modules, events, permissions, settings, telemetry, automation, services, UI sync, plugins, and diagnostics.</p>
        </div>
        <button className="primaryAction"><ShieldCheck size={18} />Inspect Core</button>
      </section>
      <section className="metricGrid securityMetrics">
        {aegisCoreCards.map((card) => {
          const Icon = card.icon;
          return (
            <article className={`metricCard ${card.tone}`} key={card.label}>
              <Icon size={21} />
              <span>{card.label}</span>
              <strong>{card.value}</strong>
              <small>{card.detail}</small>
            </article>
          );
        })}
      </section>
    </div>
  );
}

function SystemGraphPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="graphSurface">
        <div className="graphGrid" />
        {systemGraphNodes.map((node) => (
          <div
            className={node.risk >= 60 ? 'graphNode hot' : 'graphNode'}
            key={node.name}
            style={{ '--x': `${node.x}%`, '--y': `${node.y}%` } as CSSProperties}
          >
            <strong>{node.name}</strong>
            <span>{node.kind}</span>
            <small>Risk {node.risk}</small>
          </div>
        ))}
      </section>
      <section className="actionPanel">
        <h2>Relationship Trace</h2>
        {systemGraphEdges.map((edge) => (
          <div className="actionRow" key={edge}>
            <Layers3 size={16} />
            <span>{edge}</span>
          </div>
        ))}
      </section>
    </div>
  );
}

function OrchestrationPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="ruleTable orchestrationTable">
        {orchestrationPlans.map((plan) => (
          <div className="tableRow orchestrationRow" key={plan.trigger}>
            <RefreshCw size={18} />
            <strong>{plan.trigger}</strong>
            <span>{plan.steps}</span>
            <small>{plan.review}</small>
          </div>
        ))}
      </section>
      <section className="riskPanel">
        <ShieldCheck size={48} />
        <h2>Reviewable Automation</h2>
        <div className="riskNumber">100</div>
        <span className="riskBadge medium">transparent</span>
      </section>
    </div>
  );
}

function PredictiveRiskPage() {
  const score = Math.min(100, predictiveRiskSignals.reduce((total, item) => total + item.score, 0));
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <Gauge size={48} />
        <h2>Predictive Risk</h2>
        <div className="riskNumber">{score}</div>
        <span className="riskBadge high">rising</span>
      </section>
      <section className="ruleTable riskTable">
        {predictiveRiskSignals.map((signal) => (
          <div className="tableRow riskIntelRow" key={signal.category}>
            <Activity size={18} />
            <strong>{signal.category}</strong>
            <span>{signal.score}</span>
            <span>{signal.trend}</span>
            <small>{signal.detail}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function AiOpsPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="actionPanel">
        <h2>AI Operations Layer</h2>
        {aiOpsCapabilities.map((item) => (
          <div className="actionRow" key={item.capability}>
            <Activity size={16} />
            <span><strong>{item.capability}</strong> - {item.example}</span>
          </div>
        ))}
      </section>
      <section className="statusBand compactBand">
        <div className="statusRing"><span>0</span><small>auto</small></div>
        <div className="statusCopy">
          <h2>Advisory Control</h2>
          <p>AI explains incidents, logs, processes, health, and posture. It guides decisions without taking hidden action.</p>
        </div>
      </section>
    </div>
  );
}

function CommandOsPage() {
  return (
    <div className="pageGrid">
      <section className="commandHero commandOsHero">
        <div className="statusRing large">
          <span>6</span>
          <small>views</small>
        </div>
        <div className="statusCopy">
          <h2>Aegis Command Center</h2>
          <p>A modular operating surface for system maps, floating panels, live event streams, timeline replay, AI context, and advanced filtering.</p>
        </div>
      </section>
      <section className="moduleMatrix">
        {commandWidgets.map((widget) => (
          <article className="moduleCard" key={widget.name}>
            <Radar size={20} />
            <strong>{widget.name}</strong>
            <span>{widget.state}</span>
            <small>{widget.detail}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function SyncPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable syncTable">
        {syncMatrix.map((item) => (
          <div className="tableRow syncRow" key={item.product}>
            <Cloud size={18} />
            <strong>{item.product}</strong>
            <span>{item.shared}</span>
            <small>{item.mode}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function ObservabilityPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable observabilityTable">
        {observabilityRows.map((row) => (
          <div className="tableRow observabilityRow" key={row.area}>
            <Gauge size={18} />
            <strong>{row.area}</strong>
            <span>{row.value}</span>
            <span>{row.status}</span>
            <small>{row.source}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function WorkspacesPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {workspaceCards.map((workspace) => (
          <article className="moduleCard" key={workspace.name}>
            <Database size={20} />
            <strong>{workspace.name}</strong>
            <span>{workspace.layout}</span>
            <small>{workspace.widgets} - Automation {workspace.automation}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function VisualIntelPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="visualPulse">
        <div className="pulseRing one" />
        <div className="pulseRing two" />
        <div className="pulseCore">
          <strong>Visual Intelligence</strong>
          <span>Live risk pulse</span>
        </div>
      </section>
      <section className="ruleTable visualTable">
        {visualIntelligenceSurfaces.map((surface) => (
          <div className="tableRow visualRow" key={surface.surface}>
            <Activity size={18} />
            <strong>{surface.surface}</strong>
            <span>{surface.live}</span>
            <small>{surface.source}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function DistributedPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {distributedCapabilities.map((capability) => (
          <article className="moduleCard" key={capability.mode}>
            <Cloud size={20} />
            <strong>{capability.mode}</strong>
            <span>{capability.status}</span>
            <small>{capability.detail}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function SdkPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable sdkTable">
        {developerApis.map((api) => (
          <div className="tableRow sdkRow" key={api.api}>
            <FileSearch size={18} />
            <strong>{api.api}</strong>
            <span>{api.version}</span>
            <span>{api.permissions}</span>
            <small>{api.stability}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function TransparencyPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <LockKeyhole size={48} />
        <h2>Transparency Score</h2>
        <div className="riskNumber">100</div>
        <span className="riskBadge medium">visible</span>
      </section>
      <section className="ruleTable trustTable">
        {transparencyControls.map((control) => (
          <div className="tableRow trustRow" key={control.name}>
            <CheckCircle2 size={18} />
            <strong>{control.name}</strong>
            <span>{control.enabled}</span>
            <small>{control.detail}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function PremiumOsPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {premiumOsPillars.map((pillar) => (
          <article className="moduleCard" key={pillar.area}>
            <ShieldPlus size={20} />
            <strong>{pillar.area}</strong>
            <span>{pillar.status}</span>
            <small>{pillar.detail}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function AegisOsPage() {
  return (
    <div className="pageGrid">
      <section className="commandHero">
        <div className="statusRing large">
          <span>80</span>
          <small>phase</small>
        </div>
        <div className="statusCopy">
          <h2>Aegis OS Vision</h2>
          <p>A modular security and intelligence platform for endpoint protection, diagnostics, automation, AI assistance, infrastructure visibility, and premium desktop engineering.</p>
        </div>
      </section>
      <section className="actionPanel">
        <h2>Operating Environment Direction</h2>
        {aegisOsVision.map((item) => (
          <div className="actionRow" key={item}>
            <CheckCircle2 size={16} />
            <span>{item}</span>
          </div>
        ))}
      </section>
    </div>
  );
}

function ModulesPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {moduleCards.map((module) => (
          <article className="moduleCard" key={module.name}>
            <Layers3 size={20} />
            <strong>{module.name}</strong>
            <span>{module.status}</span>
            <small>v{module.version} - API {module.api} - {module.hotReload}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function LayersPage() {
  return (
    <div className="pageGrid">
      <section className="layerGrid">
        {protectionLayers.map((layer) => (
          <article className="layerCard" key={layer.layer}>
            <div className="layerOrb">{layer.score}</div>
            <span>{layer.layer}</span>
            <strong>{layer.name}</strong>
            <small>{layer.status}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function SecurityCenterPage() {
  return (
    <div className="pageGrid securityGrid">
      <section className="commandHero">
        <div className="statusRing large">
          <span>96</span>
          <small>score</small>
        </div>
        <div className="statusCopy">
          <h2>Security Center</h2>
          <p>Active protection is healthy, signed updates are verified, and recent high-risk activity is contained pending review.</p>
        </div>
        <button className="primaryAction"><ShieldPlus size={18} />Review Actions</button>
      </section>
      <section className="metricGrid securityMetrics">
        {securityCenterCards.map((card) => {
          const Icon = card.icon;
          return (
            <article className={`metricCard ${card.tone}`} key={card.label}>
              <Icon size={21} />
              <span>{card.label}</span>
              <strong>{card.value}</strong>
              <small>{card.detail}</small>
            </article>
          );
        })}
      </section>
      <section className="actionPanel">
        <h2>Recommended Actions</h2>
        {recommendedActions.map((action) => (
          <div className="actionRow" key={action}>
            <CheckCircle2 size={16} />
            <span>{action}</span>
          </div>
        ))}
      </section>
      <LiveFeed />
    </div>
  );
}

function OperationsPage() {
  return (
    <div className="pageGrid securityGrid">
      <section className="commandHero">
        <div className="statusRing large">
          <span>92</span>
          <small>ops</small>
        </div>
        <div className="statusCopy">
          <h2>Security Operations Center</h2>
          <p>A unified command view for incidents, protection layers, automation, device health, network activity, and ecosystem status.</p>
        </div>
        <button className="primaryAction"><Activity size={18} />Open Incidents</button>
      </section>
      <section className="metricGrid securityMetrics">
        {operationsCards.map((card) => {
          const Icon = card.icon;
          return (
            <article className={`metricCard ${card.tone}`} key={card.label}>
              <Icon size={21} />
              <span>{card.label}</span>
              <strong>{card.value}</strong>
              <small>{card.detail}</small>
            </article>
          );
        })}
      </section>
      <ThreatTimeline />
      <LiveFeed />
    </div>
  );
}

function ReliabilityPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {reliabilityCards.map((card) => (
          <article className="moduleCard" key={card.name}>
            <Gauge size={20} />
            <strong>{card.name}</strong>
            <span>{card.status}</span>
            <small>{card.detail}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function EventsPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable eventTable">
        {eventBusRows.map((event) => (
          <div className="tableRow eventRow" key={event.event}>
            <Database size={18} />
            <strong>{event.event}</strong>
            <span>{event.source}</span>
            <span>{event.severity}</span>
            <small>{event.route}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function AutomationPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="ruleTable automationTable">
        {automationWorkflows.map((workflow) => (
          <div className="tableRow automationRow" key={workflow.name}>
            <RefreshCw size={18} />
            <strong>{workflow.name}</strong>
            <span>{workflow.trigger}</span>
            <small>{workflow.response}</small>
          </div>
        ))}
      </section>
      <section className="riskPanel">
        <ShieldCheck size={48} />
        <h2>Safe Automation</h2>
        <div className="riskNumber">0</div>
        <span className="riskBadge medium">blind blocks</span>
      </section>
    </div>
  );
}

function AlertsPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {notificationRows.map((row) => (
          <article className="moduleCard" key={row.mode}>
            <Bell size={20} />
            <strong>{row.mode}</strong>
            <span>{row.behavior}</span>
            <small>{row.digest}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function ProfilesPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable profileTable">
        {advancedProfiles.map((profile) => (
          <div className="tableRow profileRow" key={profile.name}>
            <Settings size={18} />
            <strong>{profile.name}</strong>
            <span>{profile.ui}</span>
            <span>{profile.depth}</span>
            <small>{profile.notifications}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function KnowledgePage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable knowledgeTable">
        {knowledgeEntries.map((entry) => (
          <div className="tableRow knowledgeRow" key={entry.detection}>
            <FileSearch size={18} />
            <strong>{entry.detection}</strong>
            <span>{entry.severity}</span>
            <span>{entry.risk}</span>
            <small>{entry.remediation}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function AssistantPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="actionPanel">
        <h2>AI Security Assistant</h2>
        {assistantWorkflows.map((workflow) => (
          <div className="actionRow" key={workflow.task}>
            <Activity size={16} />
            <span><strong>{workflow.task}</strong> - {workflow.sample}</span>
          </div>
        ))}
      </section>
      <section className="statusBand compactBand">
        <div className="statusRing"><span>0</span><small>blocks</small></div>
        <div className="statusCopy">
          <h2>Educate, Then Decide</h2>
          <p>The assistant explains evidence and recommends next steps, while protection decisions stay in policies and user controls.</p>
        </div>
      </section>
    </div>
  );
}

function MarketplacePage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {marketplaceListings.map((listing) => (
          <article className="moduleCard" key={listing.name}>
            <Cloud size={20} />
            <strong>{listing.name}</strong>
            <span>{listing.trust}</span>
            <small>{listing.publisher} - {listing.permissions}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function TrustPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <LockKeyhole size={48} />
        <h2>Trust Score</h2>
        <div className="riskNumber">100</div>
        <span className="riskBadge medium">transparent</span>
      </section>
      <section className="ruleTable trustTable">
        {trustControls.map((control) => (
          <div className="tableRow trustRow" key={control.control}>
            <CheckCircle2 size={18} />
            <strong>{control.control}</strong>
            <span>{control.state}</span>
            <small>{control.detail}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function BrandPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {brandPillars.map((pillar) => (
          <article className="moduleCard" key={pillar.name}>
            <ShieldPlus size={20} />
            <strong>{pillar.name}</strong>
            <span>Brand pillar</span>
            <small>{pillar.detail}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function PlatformPage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable platformTable">
        {platformLayers.map((layer) => (
          <div className="tableRow platformRow" key={layer.layer}>
            <Layers3 size={18} />
            <strong>{layer.layer}</strong>
            <span>{layer.status}</span>
            <small>{layer.note}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function VisionPage() {
  return (
    <div className="pageGrid">
      <section className="commandHero">
        <div className="statusRing large">
          <span>65</span>
          <small>phase</small>
        </div>
        <div className="statusCopy">
          <h2>Long-Term Evolution</h2>
          <p>Aegis grows into a complete endpoint protection ecosystem: diagnostics, automation, network visibility, AI assistance, and premium engineering in one environment.</p>
        </div>
      </section>
      <section className="actionPanel">
        <h2>Strategic Direction</h2>
        {visionItems.map((item) => (
          <div className="actionRow" key={item}>
            <CheckCircle2 size={16} />
            <span>{item}</span>
          </div>
        ))}
      </section>
    </div>
  );
}

function Dashboard() {
  return (
    <div className="pageGrid dashboardGrid">
      <section className="statusBand">
        <div className="statusRing">
          <span>96</span>
          <small>health</small>
        </div>
        <div className="statusCopy">
          <h2>Protected</h2>
          <p>Realtime shield, behavior monitor, ransomware guard, and update verifier are online.</p>
        </div>
        <button className="primaryAction"><Radar size={18} />Quick Scan</button>
      </section>
      <section className="metricGrid">
        {healthCards.map((card) => {
          const Icon = card.icon;
          return (
            <article className={`metricCard ${card.tone}`} key={card.label}>
              <Icon size={21} />
              <span>{card.label}</span>
              <strong>{card.value}</strong>
              <small>{card.detail}</small>
            </article>
          );
        })}
      </section>
      <ThreatTimeline />
      <LiveFeed />
    </div>
  );
}

function ScanPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="scanRadar">
        <div className="radarSweep" />
        <Radar size={80} />
        <h2>Quick Scan Running</h2>
        <progress value="62" max="100" />
        <div className="scanStats">
          <span>18,431 files</span>
          <span>2 findings</span>
          <span>62%</span>
        </div>
      </section>
      <section className="modeList">
        {scanModes.map((mode) => (
          <button className={mode.active ? 'modeRow active' : 'modeRow'} key={mode.name}>
            <FileSearch size={18} />
            <strong>{mode.name}</strong>
            <span>{mode.scope}</span>
            <small>{mode.eta}</small>
          </button>
        ))}
      </section>
    </div>
  );
}

function IncidentsPage() {
  return (
    <div className="pageGrid incidentGrid">
      <section className="incidentPanel">
        <div className="incidentHeader">
          <ShieldAlert size={34} />
          <div>
            <h2>{incidentSummary.detection}</h2>
            <span>{incidentSummary.id}</span>
          </div>
          <strong>{incidentSummary.severity}</strong>
        </div>
        <dl>
          <dt>Path</dt>
          <dd>{incidentSummary.path}</dd>
          <dt>SHA256</dt>
          <dd>{incidentSummary.sha256}</dd>
          <dt>Action</dt>
          <dd>{incidentSummary.action}</dd>
        </dl>
        <div className="exportButtons">
          <button><FileJson size={16} />JSON</button>
          <button><FileSearch size={16} />PDF</button>
          <button><Cloud size={16} />HTML</button>
        </div>
      </section>
      <section className="timeline detailed">
        <h2>Threat Timeline</h2>
        {incidentTimeline.map((event) => (
          <div className="timelineItem detailed" key={`${event.time}-${event.event}`}>
            <CheckCircle2 size={16} />
            <strong>{event.time}</strong>
            <span>{event.event}</span>
            <small>{event.detail}</small>
          </div>
        ))}
      </section>
      <section className="actionPanel">
        <h2>Recommended Next Steps</h2>
        {incidentSummary.nextSteps.map((step) => (
          <div className="actionRow" key={step}>
            <CheckCircle2 size={16} />
            <span>{step}</span>
          </div>
        ))}
      </section>
      <section className="actionPanel">
        <h2>False Positive Workflow</h2>
        {['Mark as trusted', 'Restore and exclude', 'Submit report with notes', 'Keep local allowlist'].map((item) => (
          <button className="workflowButton" key={item}>{item}</button>
        ))}
      </section>
    </div>
  );
}

function BehaviorPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="ruleTable">
        {behaviorRules.map((rule) => (
          <div className="tableRow" key={rule.rule}>
            <Activity size={18} />
            <strong>{rule.rule}</strong>
            <span>{rule.score}</span>
            <span>{rule.action}</span>
            <small>{rule.status}</small>
          </div>
        ))}
      </section>
      <section className="riskPanel">
        <Gauge size={44} />
        <h2>Behavior Score</h2>
        <div className="riskNumber">41</div>
        <span className="riskBadge medium">Medium</span>
      </section>
    </div>
  );
}

function ProcessesPage() {
  return (
    <div className="pageGrid">
      <section className="processSurface">
        <ProcessBranch node={processTree} depth={0} />
      </section>
    </div>
  );
}

function ProcessBranch({
  node,
  depth
}: {
  node: ProcessNodeUi;
  depth: number;
}) {
  const isElevated = node.score >= 60;
  return (
    <div className="processGroup" style={{ '--depth': depth } as CSSProperties}>
      <div className={isElevated ? 'processRow elevated' : 'processRow'}>
        <span className="treeRail" />
        <Zap size={16} />
        <strong>{node.name}</strong>
        <span>PID {node.pid}</span>
        <small>Risk {node.score}</small>
      </div>
      {node.children.map((child) => (
        <ProcessBranch key={`${child.name}-${child.pid}`} node={child} depth={depth + 1} />
      ))}
    </div>
  );
}

function DiagnosticsPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="metricGrid compactMetrics">
        {diagnosticsMetrics.map((item) => (
          <article className="metricCard steady" key={item.label}>
            <Gauge size={21} />
            <span>{item.label}</span>
            <strong>{item.value}</strong>
            <small>{item.detail}</small>
          </article>
        ))}
      </section>
      <section className="ruleTable">
        {diagnosticsRows.map((row) => (
          <div className="tableRow releaseRow" key={row.name}>
            <Activity size={18} />
            <strong>{row.name}</strong>
            <span>{row.status}</span>
            <small>{row.risk}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function NetworkPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="ruleTable">
        {networkConnections.map((connection) => (
          <div className="tableRow networkRow" key={`${connection.process}-${connection.remote}`}>
            <Cloud size={18} />
            <strong>{connection.process}</strong>
            <span>{connection.remote}</span>
            <span>{connection.state}</span>
            <small>{connection.bandwidth}</small>
          </div>
        ))}
      </section>
      <section className="actionPanel">
        <h2>Safe Controls</h2>
        {firewallActions.map((action) => (
          <button className="workflowButton" key={action}>{action}</button>
        ))}
      </section>
    </div>
  );
}

function RansomwarePage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="folderPanel">
        {['Documents', 'Desktop', 'Pictures', 'Finance Archive'].map((folder) => (
          <div className="folderRow" key={folder}>
            <LockKeyhole size={18} />
            <strong>{folder}</strong>
            <span>Protected</span>
          </div>
        ))}
      </section>
      <section className="riskPanel danger">
        <ShieldAlert size={48} />
        <h2>Rename Burst</h2>
        <div className="riskNumber">73</div>
        <span className="riskBadge high">High</span>
      </section>
    </div>
  );
}

function SandboxPage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <LockKeyhole size={48} />
        <h2>Lightweight Sandbox</h2>
        <div className="riskNumber">90</div>
        <span className="riskBadge medium">seconds</span>
      </section>
      <section className="metricGrid compactMetrics">
        {sandboxFacts.map((fact) => (
          <article className="metricCard steady" key={fact.label}>
            <ShieldCheck size={21} />
            <span>{fact.label}</span>
            <strong>{fact.value}</strong>
            <small>Harmless simulation mode</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function AiPage() {
  return (
    <div className="pageGrid">
      <section className="actionPanel">
        <h2>AI-Assisted Explanations</h2>
        {aiExplanations.map((explanation) => (
          <div className="actionRow" key={explanation}>
            <Activity size={16} />
            <span>{explanation}</span>
          </div>
        ))}
      </section>
      <section className="statusBand">
        <div className="statusRing"><span>0</span><small>blocks</small></div>
        <div className="statusCopy">
          <h2>Explain-Only Mode</h2>
          <p>AI summarizes evidence, classifications, and recommendations. It never blocks by itself.</p>
        </div>
      </section>
    </div>
  );
}

function PluginsPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {pluginCards.map((plugin) => (
          <article className="moduleCard" key={plugin.name}>
            <Settings size={20} />
            <strong>{plugin.name}</strong>
            <span>{plugin.status}</span>
            <small>{plugin.permission}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function PowerToolsPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix toolMatrix">
        {powerTools.map((tool) => (
          <button className="settingsProfile toolButton" key={tool}>
            <FileSearch size={20} />
            <strong>{tool}</strong>
            <span>Power user workspace</span>
          </button>
        ))}
      </section>
    </div>
  );
}

function ScorePage() {
  const score = Math.round(scoreBreakdown.reduce((total, item) => total + item.score, 0) / scoreBreakdown.length);
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <Gauge size={48} />
        <h2>Aegis Security Score</h2>
        <div className="riskNumber">{score}</div>
        <span className="riskBadge medium">/100</span>
      </section>
      <section className="ruleTable">
        {scoreBreakdown.map((item) => (
          <div className="tableRow scoreRow" key={item.category}>
            <CheckCircle2 size={18} />
            <strong>{item.category}</strong>
            <span>{item.score}</span>
            <small>{item.deduction}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function EcosystemPage() {
  return (
    <div className="pageGrid">
      <section className="moduleMatrix">
        {ecosystemIntegrations.map((item) => (
          <article className={item.enabled ? 'moduleCard' : 'moduleCard disabled'} key={item.product}>
            <Cloud size={20} />
            <strong>{item.product}</strong>
            <span>{item.enabled ? 'Connected' : 'Planned'}</span>
            <small>{item.status}</small>
          </article>
        ))}
      </section>
    </div>
  );
}

function QuarantinePage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="ruleTable">
        {quarantineItems.map((item) => (
          <div className="tableRow quarantine" key={item.hash}>
            <ArchiveRestore size={18} />
            <strong>{item.name}</strong>
            <span>{item.detection}</span>
            <small>{item.action}</small>
          </div>
        ))}
      </section>
      <section className="detailDrawer">
        <h2>Incident Report</h2>
        <dl>
          <dt>Detection</dt>
          <dd>Suspicious.File.Heuristic.65</dd>
          <dt>Hash</dt>
          <dd>91b6...a22f</dd>
          <dt>Restore</dt>
          <dd>Requires confirmation</dd>
        </dl>
      </section>
    </div>
  );
}

function ThreatIntelPage() {
  return (
    <div className="pageGrid">
      <section className="metricGrid securityMetrics">
        {threatIntel.map((item) => (
          <article className="metricCard steady" key={item.label}>
            <Activity size={21} />
            <span>{item.label}</span>
            <strong>{item.value}</strong>
            <small>{item.detail}</small>
          </article>
        ))}
      </section>
      <section className="timeline">
        <h2>Update Changelog</h2>
        {['Added behavior scoring thresholds', 'Improved quarantine restore validation', 'Added signed manifest verification', 'Added false positive allowlist contracts'].map((item, index) => (
          <div className="timelineItem" key={item}>
            <CheckCircle2 size={16} />
            <span>{item}</span>
            <small>v{index + 1}</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function EnterprisePage() {
  return (
    <div className="pageGrid">
      <section className="ruleTable enterprise">
        {enterpriseDevices.map((device) => (
          <div className="tableRow enterpriseRow" key={device.name}>
            <Building2 size={18} />
            <strong>{device.name}</strong>
            <span>Health {device.health}</span>
            <span>{device.policy}</span>
            <small>{device.threats} threats</small>
          </div>
        ))}
      </section>
    </div>
  );
}

function ReleasePage() {
  return (
    <div className="pageGrid twoColumn">
      <section className="riskPanel">
        <ShieldPlus size={48} />
        <h2>Alpha Release</h2>
        <div className="riskNumber">0.1</div>
        <span className="riskBadge medium">Validated</span>
      </section>
      <section className="ruleTable">
        {releaseChecklist.map((item) => (
          <div className="tableRow releaseRow" key={item.item}>
            <CheckCircle2 size={18} />
            <strong>{item.item}</strong>
            <span>{item.status}</span>
          </div>
        ))}
      </section>
    </div>
  );
}

function UpdatesPage() {
  return (
    <div className="pageGrid">
      <section className="updateList">
        {updateHistory.map((item) => {
          const Icon = item.icon;
          return (
            <div className="updateRow" key={item.version}>
              <Icon size={18} />
              <strong>{item.version}</strong>
              <span>{item.channel}</span>
              <small>{item.status}</small>
            </div>
          );
        })}
      </section>
    </div>
  );
}

function LogsPage() {
  return (
    <div className="pageGrid">
      <LiveFeed />
    </div>
  );
}

function SettingsPage() {
  return (
    <div className="pageGrid twoColumn">
      {settingsProfiles.map((profile) => {
        const Icon = profile.icon;
        return (
          <button className="settingsProfile" key={profile.name}>
            <Icon size={20} />
            <strong>{profile.name}</strong>
            <span>{profile.detail}</span>
          </button>
        );
      })}
    </div>
  );
}

function ThreatTimeline() {
  return (
    <section className="timeline">
      <h2>Threat Timeline</h2>
      {['Hash match', 'Behavior spike', 'Quarantine', 'Manifest verified'].map((item, index) => (
        <div className="timelineItem" key={item}>
          <CheckCircle2 size={16} />
          <span>{item}</span>
          <small>{12 - index * 2}:0{index}</small>
        </div>
      ))}
    </section>
  );
}

function LiveFeed() {
  return (
    <section className="liveFeed">
      <h2>Live Event Feed</h2>
      {liveEvents.map((event) => (
        <div className={`feedRow ${event.level}`} key={`${event.time}-${event.text}`}>
          <span>{event.time}</span>
          <strong>{event.level}</strong>
          <p>{event.text}</p>
        </div>
      ))}
    </section>
  );
}

function headlineFor(page: PageKey) {
  const headlines: Record<PageKey, string> = {
    security: 'Security Center',
    deviceControl: 'Device Guard Control',
    aegisCore: 'Unified AegisCore',
    systemGraph: 'Live System Graph',
    orchestration: 'Intelligent Orchestration',
    predictiveRisk: 'Predictive Risk Engine',
    aiOps: 'AI Operations Layer',
    commandOs: 'Aegis Command OS',
    sync: 'Ecosystem Synchronization',
    observability: 'Advanced Observability',
    workspaces: 'Workspace System',
    visualIntel: 'Visual Intelligence',
    distributed: 'Distributed Architecture',
    sdk: 'Developer Ecosystem',
    transparency: 'Trust And Transparency',
    premiumOs: 'Premium Brand Experience',
    aegisOs: 'Aegis OS Vision',
    operations: 'Security Operations',
    reliability: 'Reliability Engineering',
    events: 'Unified Event Bus',
    automation: 'Workflow Automation',
    alerts: 'Notification Center',
    profiles: 'User Profiles',
    knowledge: 'Threat Knowledge',
    assistant: 'AI Security Assistant',
    marketplace: 'Plugin Marketplace',
    trust: 'Reliability And Trust',
    brand: 'Brand Identity',
    platform: 'Cross Platform Architecture',
    vision: 'Long-Term Vision',
    modules: 'Modular Architecture',
    layers: 'Protection Layers',
    scan: 'Scan Operations',
    incidents: 'Incident Reports',
    behavior: 'Behavioral Monitoring',
    processes: 'Process Tree',
    diagnostics: 'System Diagnostics',
    network: 'Firewall Visibility',
    ransomware: 'Protected Folders',
    sandbox: 'Secure Sandbox',
    ai: 'AI-Assisted Detection',
    plugins: 'Plugin System',
    tools: 'Developer Tools',
    score: 'Security Score',
    ecosystem: 'Aegis Ecosystem',
    quarantine: 'Quarantine Vault',
    intel: 'Threat Intelligence',
    enterprise: 'Admin Console',
    release: 'Release Engineering',
    updates: 'Update Trust',
    logs: 'Security Logs',
    settings: 'Policy Profiles'
  };

  return headlines[page];
}
