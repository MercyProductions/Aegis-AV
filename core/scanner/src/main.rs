use aegis_scanner::{
    resolve_profile_targets, ScanEngine, ScanOptions, ScanProfile, ScanSummary, ScanVerdict,
    SignatureDb,
};
use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "aegis-scanner")]
#[command(about = "Aegis defensive antivirus scanner CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan {
        #[arg(value_name = "PATH")]
        target: Option<PathBuf>,

        #[arg(long, value_enum, default_value_t = CliProfile::Custom)]
        profile: CliProfile,

        #[arg(long, default_value = "core/signatures/hashes.json")]
        signatures: PathBuf,

        #[arg(long, default_value_t = 256)]
        max_file_size_mb: u64,

        #[arg(long)]
        json: bool,

        #[arg(long)]
        fail_on_detect: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliProfile {
    Quick,
    Full,
    Deep,
    Custom,
}

impl From<CliProfile> for ScanProfile {
    fn from(value: CliProfile) -> Self {
        match value {
            CliProfile::Quick => Self::Quick,
            CliProfile::Full => Self::Full,
            CliProfile::Deep => Self::Deep,
            CliProfile::Custom => Self::Custom,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            target,
            profile,
            signatures,
            max_file_size_mb,
            json,
            fail_on_detect,
        } => {
            let signature_path = resolve_signature_path(&signatures).with_context(|| {
                format!(
                    "could not find signature database: {}",
                    signatures.display()
                )
            })?;
            let signatures = SignatureDb::load_json(&signature_path).with_context(|| {
                format!(
                    "failed to load signatures from {}",
                    signature_path.display()
                )
            })?;
            let options = ScanOptions {
                profile: profile.into(),
                max_file_size_bytes: max_file_size_mb.saturating_mul(1024 * 1024),
            };
            let engine = ScanEngine::new(signatures, options);
            let plan = resolve_profile_targets(profile.into(), target);
            let summary = engine.scan_paths(PathBuf::from(&plan.label), &plan.targets);

            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                print_human_summary(&summary);
            }

            if fail_on_detect && summary.threats_found > 0 {
                std::process::exit(2);
            }

            if summary.errors > 0 {
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn resolve_signature_path(path: &Path) -> Option<PathBuf> {
    if path.exists() {
        return Some(path.to_path_buf());
    }

    let current_dir_candidate = std::env::current_dir().ok()?.join(path);
    if current_dir_candidate.exists() {
        return Some(current_dir_candidate);
    }

    let exe = std::env::current_exe().ok()?;
    for ancestor in exe.ancestors() {
        let candidate = ancestor.join(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn print_human_summary(summary: &ScanSummary) {
    println!("Aegis Scanner Summary");
    println!("Target: {}", summary.target.display());
    println!("Profile: {:?}", summary.profile);
    println!("Files scanned: {}", summary.files_scanned);
    println!("Files skipped: {}", summary.files_skipped);
    println!("Suspicious: {}", summary.suspicious_found);
    println!("Threats: {}", summary.threats_found);
    println!("Errors: {}", summary.errors);
    println!("Duration: {} ms", summary.duration_ms);

    for result in &summary.results {
        if matches!(result.verdict, ScanVerdict::Clean) {
            continue;
        }

        println!();
        println!("[{:?}] {}", result.verdict, result.path.display());
        if let Some(name) = &result.detection_name {
            println!("Detection: {name}");
        }
        if let Some(rule) = &result.matched_rule {
            println!("Rule: {rule}");
        }
        if let Some(sha256) = &result.sha256 {
            println!("SHA256: {sha256}");
        }
        for finding in &result.heuristics {
            println!(
                "Heuristic: {} (+{}) {}",
                finding.rule_id, finding.score, finding.description
            );
        }
        for error in &result.errors {
            println!("Error: {error}");
        }
    }
}
