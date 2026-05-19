use crate::types::ScanProfile;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanTargetPlan {
    pub profile: ScanProfile,
    pub label: String,
    pub targets: Vec<PathBuf>,
    pub includes_running_processes: bool,
    pub includes_archives: bool,
    pub includes_deep_metadata: bool,
}

pub fn resolve_profile_targets(
    profile: ScanProfile,
    custom_target: Option<PathBuf>,
) -> ScanTargetPlan {
    match profile {
        ScanProfile::Custom => ScanTargetPlan {
            profile,
            label: "Custom Scan".to_string(),
            targets: vec![custom_target.unwrap_or_else(|| PathBuf::from("."))],
            includes_running_processes: false,
            includes_archives: false,
            includes_deep_metadata: false,
        },
        ScanProfile::Quick => ScanTargetPlan {
            profile,
            label: "Quick Scan".to_string(),
            targets: existing_paths(quick_scan_candidates()),
            includes_running_processes: true,
            includes_archives: false,
            includes_deep_metadata: false,
        },
        ScanProfile::Full => ScanTargetPlan {
            profile,
            label: "Full Scan".to_string(),
            targets: existing_paths(full_scan_candidates()),
            includes_running_processes: true,
            includes_archives: false,
            includes_deep_metadata: true,
        },
        ScanProfile::Deep => ScanTargetPlan {
            profile,
            label: "Deep Scan".to_string(),
            targets: existing_paths(full_scan_candidates()),
            includes_running_processes: true,
            includes_archives: true,
            includes_deep_metadata: true,
        },
    }
}

fn quick_scan_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(user_profile) = env_path("USERPROFILE") {
        paths.push(user_profile.join("Downloads"));
        paths.push(user_profile.join("Desktop"));
        paths.push(user_profile.join("Documents"));
        paths.push(
            user_profile.join("AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup"),
        );
    }
    if let Some(program_data) = env_path("ProgramData") {
        paths.push(program_data.join("Microsoft/Windows/Start Menu/Programs/Startup"));
    }
    if let Some(temp) = env_path("TEMP").or_else(|| env_path("TMP")) {
        paths.push(temp);
    }

    paths
}

fn full_scan_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(system_drive) = env_path("SystemDrive") {
        paths.push(system_drive);
    }
    if let Some(user_profile) = env_path("USERPROFILE") {
        paths.push(user_profile);
    }
    if let Some(program_files) = env_path("ProgramFiles") {
        paths.push(program_files);
    }
    if let Some(program_files_x86) = env_path("ProgramFiles(x86)") {
        paths.push(program_files_x86);
    }
    if let Some(program_data) = env_path("ProgramData") {
        paths.push(program_data);
    }

    paths
}

fn existing_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths.into_iter().filter(|path| path.exists()) {
        if !deduped.iter().any(|existing: &PathBuf| existing == &path) {
            deduped.push(path);
        }
    }
    if deduped.is_empty() {
        deduped.push(PathBuf::from("."));
    }
    deduped
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_scan_defaults_to_current_directory() {
        let plan = resolve_profile_targets(ScanProfile::Custom, None);

        assert_eq!(plan.targets, vec![PathBuf::from(".")]);
        assert!(!plan.includes_archives);
    }

    #[test]
    fn deep_scan_enables_deeper_inspection_flags() {
        let plan = resolve_profile_targets(ScanProfile::Deep, None);

        assert!(plan.includes_archives);
        assert!(plan.includes_running_processes);
        assert!(plan.includes_deep_metadata);
    }
}
