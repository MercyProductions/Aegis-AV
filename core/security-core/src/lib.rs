pub mod aegis_core;
pub mod ai_assist;
pub mod ai_ops;
pub mod assistant;
pub mod automation;
pub mod behavior;
pub mod brand;
pub mod cloud_sync;
pub mod developer_ecosystem;
pub mod diagnostics;
pub mod distributed;
pub mod ecosystem;
pub mod enterprise;
pub mod event_bus;
pub mod false_positive;
pub mod firewall;
pub mod incident;
pub mod knowledge_base;
pub mod logging;
pub mod marketplace;
pub mod module_system;
pub mod notifications;
pub mod observability;
pub mod ops_dashboard;
pub mod orchestration;
pub mod performance;
pub mod platform;
pub mod plugin_system;
pub mod policies;
pub mod power_tools;
pub mod premium_experience;
pub mod process_tree;
pub mod protection_layers;
pub mod ransomware;
pub mod reputation;
pub mod risk_intelligence;
pub mod sandbox;
pub mod scoring;
pub mod self_protection;
pub mod stability;
pub mod system_graph;
pub mod threat_intel;
pub mod transparency;
pub mod trust;
pub mod update_ecosystem;
pub mod updates;
pub mod user_profiles;
pub mod visual_intelligence;
pub mod workspaces;

pub use aegis_core::{AegisCoreRuntime, CoreCapability, CoreSubsystem};
pub use ai_assist::{AiAnalysisRequest, AiAnalysisResponse, AiRecommendation};
pub use ai_ops::{AiOperationsLayer, AiOpsCapability, AiOpsRequest, AiOpsResponse};
pub use assistant::{AegisAssistant, AssistantPromptKind, AssistantRequest, AssistantResponse};
pub use automation::{AutomationEngine, AutomationWorkflow, WorkflowAction, WorkflowCondition};
pub use behavior::{BehaviorEvent, BehaviorMonitor, BehaviorRuleMatch, ResponseAction, RiskLevel};
pub use brand::BrandIdentity;
pub use cloud_sync::{CloudSyncPayload, CloudSyncSettings};
pub use developer_ecosystem::{DeveloperApi, DeveloperApiSurface, DeveloperEcosystem};
pub use diagnostics::{DiagnosticsSnapshot, ResourceUsage};
pub use distributed::{DeploymentMode, DistributedArchitecturePlan, DistributedCapability};
pub use ecosystem::{AegisIntegration, EcosystemRegistry};
pub use enterprise::{DeviceRecord, EnterpriseAdminState, RemoteScanRequest};
pub use event_bus::{AegisEvent, AegisEventKind, EventBus, EventSeverity};
pub use false_positive::{AllowList, FalsePositiveReport, TrustAction};
pub use firewall::{ConnectionAction, FirewallVisibilitySnapshot, NetworkConnection};
pub use incident::{IncidentReport, IncidentReportExporter, ThreatTimelineEvent};
pub use knowledge_base::{ThreatKnowledgeBase, ThreatKnowledgeEntry};
pub use marketplace::{MarketplaceListing, MarketplaceRegistry, MarketplaceValidation};
pub use module_system::{AegisModuleId, ModuleManifest, ModuleRegistry};
pub use notifications::{AegisNotification, AlertLevel, NotificationCenter, NotificationMode};
pub use observability::{ObservabilityMetric, ObservabilitySnapshot, PerformanceTrace};
pub use ops_dashboard::SecurityOperationsSnapshot;
pub use orchestration::{
    IntelligentAutomationEngine, OrchestrationPlan, OrchestrationStep, OrchestrationStepKind,
    OrchestrationTrigger,
};
pub use platform::{CrossPlatformPlan, PlatformCapability, PlatformLayer};
pub use plugin_system::{PluginManifest, PluginPermission, PluginRegistry};
pub use policies::{PolicyEngine, PolicyProfile, ProtectionPolicy};
pub use premium_experience::{
    PremiumExperienceArea, PremiumExperiencePillar, PremiumExperiencePlan,
};
pub use process_tree::{ProcessInfo, ProcessNode, ProcessSnapshot};
pub use protection_layers::{ProtectionLayer, ProtectionLayerId, ProtectionLayerStack};
pub use ransomware::{ProtectedFolderEvent, RansomwareMonitor};
pub use reputation::{FileReputation, ReputationDatabase, ReputationVerdict};
pub use risk_intelligence::{
    PredictiveRiskAssessment, PredictiveRiskEngine, RiskSignal, RiskSignalKind, RiskTrend,
};
pub use sandbox::{SandboxPolicy, SandboxReport, SandboxSession};
pub use scoring::{SecurityScoreBreakdown, SecurityScoreEngine};
pub use self_protection::{
    SelfProtectionCheck, SelfProtectionManifest, SelfProtectionStatus, TrustedComponent,
};
pub use stability::{
    RecoveryAction, StabilityAssessment, StabilityEngine, StabilitySeverity, StabilitySignal,
};
pub use system_graph::{
    GraphEdgeKind, GraphNodeKind, LiveSystemGraph, SystemGraphEdge, SystemGraphNode,
};
pub use threat_intel::{DetectionCategory, ThreatIntelSnapshot};
pub use transparency::{TransparencyControl, TransparencyLedger};
pub use trust::{TrustControlState, TrustScoreEngine};
pub use update_ecosystem::{DifferentialUpdate, OfflineUpdatePackage, RollbackSnapshot};
pub use user_profiles::{AegisUserProfile, UiComplexity, UserProfileEngine, UserProfileSettings};
pub use visual_intelligence::{VisualIntelligenceEngine, VisualIntelligenceSurface, VisualLayer};
pub use workspaces::{WorkspaceCatalog, WorkspaceDefinition, WorkspaceKind};
