use aegis_scanner::{ScanEngine, ScanOptions, ScanProfile, ScanVerdict, SignatureDb};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_INTERVAL_SECONDS: u64 = 30;
const DEFAULT_MAX_FILE_SIZE_MB: u64 = 256;

#[derive(Debug, Parser)]
#[command(name = "aegis-agent")]
#[command(about = "Aegis local defensive guard agent")]
struct Cli {
    #[command(subcommand)]
    command: AgentCommand,
}

#[derive(Debug, Subcommand)]
enum AgentCommand {
    /// Arm the local guard. The running agent will begin periodic scans.
    Arm {
        /// Folder to watch. Can be supplied multiple times.
        #[arg(long = "path")]
        paths: Vec<PathBuf>,
        /// Scan interval for the persistent guard loop.
        #[arg(long, default_value_t = DEFAULT_INTERVAL_SECONDS)]
        interval_seconds: u64,
    },
    /// Disarm the local guard. The running agent will pause scanning.
    Disarm,
    /// Show current guard state.
    Status {
        /// Print machine-readable JSON for GUI integrations.
        #[arg(long)]
        json: bool,
    },
    /// Run the visible foreground guard loop. Stop with Ctrl+C.
    Run {
        /// Arm immediately before starting the loop.
        #[arg(long)]
        arm: bool,
        /// Folder to watch. Can be supplied multiple times.
        #[arg(long = "path")]
        paths: Vec<PathBuf>,
        /// Scan interval for the persistent guard loop.
        #[arg(long, default_value_t = DEFAULT_INTERVAL_SECONDS)]
        interval_seconds: u64,
        /// Signature file to use.
        #[arg(long, default_value = "core/signatures/hashes.json")]
        signatures: PathBuf,
        /// Maximum file size to scan.
        #[arg(long, default_value_t = DEFAULT_MAX_FILE_SIZE_MB)]
        max_file_size_mb: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AgentState {
    armed: bool,
    interval_seconds: u64,
    watched_paths: Vec<PathBuf>,
    last_updated_unix_seconds: u64,
    last_scan_unix_seconds: Option<u64>,
    last_scan_files: usize,
    last_scan_suspicious: usize,
    last_scan_threats: usize,
    last_scan_errors: usize,
}

impl Default for AgentState {
    fn default() -> Self {
        Self {
            armed: false,
            interval_seconds: DEFAULT_INTERVAL_SECONDS,
            watched_paths: default_watch_paths(),
            last_updated_unix_seconds: now_unix_seconds(),
            last_scan_unix_seconds: None,
            last_scan_files: 0,
            last_scan_suspicious: 0,
            last_scan_threats: 0,
            last_scan_errors: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct AgentEvent {
    time_unix_seconds: u64,
    level: String,
    event: String,
    details: BTreeMap<String, String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let paths = AgentPaths::create()?;

    match cli.command {
        AgentCommand::Arm {
            paths: requested_paths,
            interval_seconds,
        } => {
            let mut state = load_state(&paths)?;
            state.armed = true;
            state.interval_seconds = interval_seconds.max(5);
            state.watched_paths = selected_paths(requested_paths);
            state.last_updated_unix_seconds = now_unix_seconds();
            save_state(&paths, &state)?;
            log_event(
                &paths,
                "info",
                "agent_armed",
                details([
                    ("interval_seconds", state.interval_seconds.to_string()),
                    ("watched_paths", display_paths(&state.watched_paths)),
                ]),
            )?;
            println!("Aegis guard armed.");
            println!("Watching: {}", display_paths(&state.watched_paths));
            println!("Interval: {} seconds", state.interval_seconds);
        }
        AgentCommand::Disarm => {
            let mut state = load_state(&paths)?;
            state.armed = false;
            state.last_updated_unix_seconds = now_unix_seconds();
            save_state(&paths, &state)?;
            log_event(&paths, "info", "agent_disarmed", BTreeMap::new())?;
            println!("Aegis guard disarmed.");
        }
        AgentCommand::Status { json } => {
            let state = load_state(&paths)?;
            if json {
                print_status_json(&state, &paths)?;
            } else {
                print_status(&state, &paths);
            }
        }
        AgentCommand::Run {
            arm,
            paths: requested_paths,
            interval_seconds,
            signatures,
            max_file_size_mb,
        } => {
            if arm {
                let mut state = load_state(&paths)?;
                state.armed = true;
                state.interval_seconds = interval_seconds.max(5);
                state.watched_paths = selected_paths(requested_paths);
                state.last_updated_unix_seconds = now_unix_seconds();
                save_state(&paths, &state)?;
            }
            run_guard_loop(&paths, &signatures, max_file_size_mb)?;
        }
    }

    Ok(())
}

fn run_guard_loop(paths: &AgentPaths, signatures_path: &Path, max_file_size_mb: u64) -> Result<()> {
    println!("Aegis guard running in the foreground. Stop with Ctrl+C.");
    println!("State: {}", paths.state_file.display());
    println!("Logs: {}", paths.log_file.display());
    log_event(paths, "info", "agent_started", BTreeMap::new())?;

    loop {
        let mut state = load_state(paths)?;
        if state.armed {
            println!(
                "[Aegis] Armed. Scanning {} path(s)...",
                state.watched_paths.len()
            );
            let summary = scan_once(&state, signatures_path, max_file_size_mb)?;
            state.last_scan_unix_seconds = Some(now_unix_seconds());
            state.last_scan_files = summary.files;
            state.last_scan_suspicious = summary.suspicious;
            state.last_scan_threats = summary.threats;
            state.last_scan_errors = summary.errors;
            save_state(paths, &state)?;
            log_event(
                paths,
                "info",
                "guard_scan_completed",
                details([
                    ("files", summary.files.to_string()),
                    ("suspicious", summary.suspicious.to_string()),
                    ("threats", summary.threats.to_string()),
                    ("errors", summary.errors.to_string()),
                ]),
            )?;
            if summary.threats > 0 || summary.suspicious > 0 {
                println!(
                    "[Aegis] Review needed: {} malicious, {} suspicious.",
                    summary.threats, summary.suspicious
                );
            } else {
                println!("[Aegis] Clean scan: {} file(s).", summary.files);
            }
        } else {
            println!("[Aegis] Disarmed. Standing by.");
        }

        thread::sleep(Duration::from_secs(state.interval_seconds.max(5)));
    }
}

fn scan_once(
    state: &AgentState,
    signatures_path: &Path,
    max_file_size_mb: u64,
) -> Result<GuardScanSummary> {
    let signatures = SignatureDb::load_json(signatures_path).with_context(|| {
        format!(
            "failed to load signature database from {}",
            signatures_path.display()
        )
    })?;
    let engine = ScanEngine::new(
        signatures,
        ScanOptions {
            profile: ScanProfile::Quick,
            max_file_size_bytes: max_file_size_mb.saturating_mul(1024 * 1024),
        },
    );
    let existing_paths = state
        .watched_paths
        .iter()
        .filter(|path| path.exists())
        .cloned()
        .collect::<Vec<_>>();

    if existing_paths.is_empty() {
        return Ok(GuardScanSummary::default());
    }

    let summary = engine.scan_paths(PathBuf::from("aegis_guard"), &existing_paths);
    for finding in summary.results.iter().filter(|result| {
        matches!(
            result.verdict,
            ScanVerdict::Suspicious | ScanVerdict::Malicious
        )
    }) {
        println!(
            "[Aegis] {:?}: {} ({})",
            finding.verdict,
            finding.path.display(),
            finding
                .detection_name
                .as_deref()
                .unwrap_or("No detection name")
        );
    }

    Ok(GuardScanSummary {
        files: summary.files_scanned,
        suspicious: summary.suspicious_found,
        threats: summary.threats_found,
        errors: summary.errors,
    })
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GuardScanSummary {
    files: usize,
    suspicious: usize,
    threats: usize,
    errors: usize,
}

#[derive(Debug, Clone)]
struct AgentPaths {
    root: PathBuf,
    state_file: PathBuf,
    log_file: PathBuf,
}

impl AgentPaths {
    fn create() -> Result<Self> {
        let root = env::var_os("ProgramData")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("data"))
            .join("AegisAV");
        let logs = root.join("Logs");
        fs::create_dir_all(&logs)
            .with_context(|| format!("failed to create {}", logs.display()))?;
        Ok(Self {
            state_file: root.join("agent-state.json"),
            log_file: logs.join("agent-events.jsonl"),
            root,
        })
    }
}

fn load_state(paths: &AgentPaths) -> Result<AgentState> {
    if !paths.state_file.exists() {
        let state = AgentState::default();
        save_state(paths, &state)?;
        return Ok(state);
    }

    let content = fs::read_to_string(&paths.state_file)
        .with_context(|| format!("failed to read {}", paths.state_file.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse {}", paths.state_file.display()))
}

fn save_state(paths: &AgentPaths, state: &AgentState) -> Result<()> {
    fs::create_dir_all(&paths.root)
        .with_context(|| format!("failed to create {}", paths.root.display()))?;
    let content = serde_json::to_string_pretty(state)?;
    fs::write(&paths.state_file, content)
        .with_context(|| format!("failed to write {}", paths.state_file.display()))
}

fn log_event(
    paths: &AgentPaths,
    level: impl Into<String>,
    event: impl Into<String>,
    details: BTreeMap<String, String>,
) -> Result<()> {
    let entry = AgentEvent {
        time_unix_seconds: now_unix_seconds(),
        level: level.into(),
        event: event.into(),
        details,
    };
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.log_file)
        .with_context(|| format!("failed to open {}", paths.log_file.display()))?;
    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

fn print_status(state: &AgentState, paths: &AgentPaths) {
    println!("Aegis guard status");
    println!("Armed: {}", state.armed);
    println!("Interval: {} seconds", state.interval_seconds);
    println!("Watching: {}", display_paths(&state.watched_paths));
    println!(
        "Last scan: {}",
        state
            .last_scan_unix_seconds
            .map(|value| value.to_string())
            .unwrap_or_else(|| "never".to_string())
    );
    println!("Last files scanned: {}", state.last_scan_files);
    println!("Last suspicious: {}", state.last_scan_suspicious);
    println!("Last threats: {}", state.last_scan_threats);
    println!("Last errors: {}", state.last_scan_errors);
    println!("State file: {}", paths.state_file.display());
    println!("Log file: {}", paths.log_file.display());
}

fn print_status_json(state: &AgentState, paths: &AgentPaths) -> Result<()> {
    let mut value = serde_json::to_value(state)?;
    if let Some(object) = value.as_object_mut() {
        object.insert(
            "state_file".to_string(),
            serde_json::Value::String(paths.state_file.display().to_string()),
        );
        object.insert(
            "log_file".to_string(),
            serde_json::Value::String(paths.log_file.display().to_string()),
        );
    }
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn selected_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    if paths.is_empty() {
        default_watch_paths()
    } else {
        paths
    }
}

fn default_watch_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(profile) = env::var_os("USERPROFILE").map(PathBuf::from) {
        paths.push(profile.join("Downloads"));
        paths.push(profile.join("Desktop"));
        paths.push(profile.join("Documents"));
    }
    if let Some(temp) = env::var_os("TEMP").map(PathBuf::from) {
        paths.push(temp);
    }
    paths
}

fn display_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("; ")
}

fn details<const N: usize>(items: [(&str, String); N]) -> BTreeMap<String, String> {
    items
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_starts_disarmed() {
        let state = AgentState::default();

        assert!(!state.armed);
        assert_eq!(state.interval_seconds, DEFAULT_INTERVAL_SECONDS);
    }

    #[test]
    fn selected_paths_uses_requested_paths() {
        let paths = selected_paths(vec![PathBuf::from("C:/Safe/Test")]);

        assert_eq!(paths, vec![PathBuf::from("C:/Safe/Test")]);
    }
}
