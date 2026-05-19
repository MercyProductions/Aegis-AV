use crate::hashing::sha256_file;
use crate::heuristics::{score_file, total_score};
use crate::signatures::SignatureDb;
use crate::types::{
    Detection, FileMetadata, HeuristicFinding, ScanProfile, ScanResult, ScanSummary, ScanVerdict,
};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub profile: ScanProfile,
    pub max_file_size_bytes: u64,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            profile: ScanProfile::Custom,
            max_file_size_bytes: 256 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScanEngine {
    signatures: SignatureDb,
    options: ScanOptions,
}

impl ScanEngine {
    pub fn new(signatures: SignatureDb, options: ScanOptions) -> Self {
        Self {
            signatures,
            options,
        }
    }

    pub fn scan_path(&self, target: &Path) -> ScanSummary {
        self.scan_paths(PathBuf::from(target), &[target.to_path_buf()])
    }

    pub fn scan_paths(&self, target_label: PathBuf, targets: &[PathBuf]) -> ScanSummary {
        let started = Instant::now();
        let mut results = Vec::new();

        for target in targets {
            if target.is_file() {
                results.push(self.scan_file(target));
            } else if target.is_dir() {
                for entry in WalkDir::new(target).follow_links(false).into_iter() {
                    match entry {
                        Ok(entry) if entry.file_type().is_file() => {
                            results.push(self.scan_file(entry.path()))
                        }
                        Ok(_) => {}
                        Err(error) => results.push(error_result(
                            error
                                .path()
                                .map(Path::to_path_buf)
                                .unwrap_or_else(|| target.to_path_buf()),
                            error.to_string(),
                        )),
                    }
                }
            } else {
                results.push(error_result(
                    target.to_path_buf(),
                    "target does not exist or is not a regular file/directory".to_string(),
                ));
            }
        }

        let files_scanned = results
            .iter()
            .filter(|result| {
                matches!(
                    result.verdict,
                    ScanVerdict::Clean | ScanVerdict::Suspicious | ScanVerdict::Malicious
                )
            })
            .count();
        let files_skipped = results
            .iter()
            .filter(|result| result.verdict == ScanVerdict::Skipped)
            .count();
        let errors = results
            .iter()
            .filter(|result| result.verdict == ScanVerdict::Error)
            .count();
        let threats_found = results
            .iter()
            .filter(|result| result.verdict == ScanVerdict::Malicious)
            .count();
        let suspicious_found = results
            .iter()
            .filter(|result| result.verdict == ScanVerdict::Suspicious)
            .count();

        ScanSummary {
            target: target_label,
            profile: self.options.profile,
            files_scanned,
            files_skipped,
            errors,
            threats_found,
            suspicious_found,
            duration_ms: started.elapsed().as_millis(),
            results,
        }
    }

    pub fn scan_file(&self, path: &Path) -> ScanResult {
        let started = Instant::now();

        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(error) => {
                return error_result_with_duration(
                    path.to_path_buf(),
                    error.to_string(),
                    started.elapsed().as_millis(),
                )
            }
        };

        let file_metadata = build_file_metadata(path, &metadata);

        if metadata.len() > self.options.max_file_size_bytes {
            return ScanResult {
                path: path.to_path_buf(),
                verdict: ScanVerdict::Skipped,
                detection_name: None,
                confidence_score: 0,
                matched_rule: None,
                sha256: None,
                file_metadata: Some(file_metadata),
                scan_duration_ms: started.elapsed().as_millis(),
                heuristics: Vec::new(),
                errors: vec![format!(
                    "file exceeds max scan size of {} bytes",
                    self.options.max_file_size_bytes
                )],
            };
        }

        let file_head = read_file_head(path, 512).unwrap_or_default();
        let sha256 = match sha256_file(path) {
            Ok(hash) => hash,
            Err(error) => {
                return error_result_with_duration(
                    path.to_path_buf(),
                    error.to_string(),
                    started.elapsed().as_millis(),
                )
            }
        };

        if let Some(signature) = self.signatures.find_sha256(&sha256) {
            let detection = Detection {
                id: signature.id.clone(),
                name: signature.name.clone(),
                severity: signature.severity.clone(),
                description: signature.description.clone(),
                recommended_action: signature.recommended_action.clone(),
            };

            return ScanResult {
                path: path.to_path_buf(),
                verdict: ScanVerdict::Malicious,
                detection_name: Some(detection.name),
                confidence_score: 100,
                matched_rule: Some(detection.id),
                sha256: Some(sha256),
                file_metadata: Some(file_metadata),
                scan_duration_ms: started.elapsed().as_millis(),
                heuristics: vec![HeuristicFinding {
                    rule_id: "SIG-KNOWN-BAD-HASH".to_string(),
                    description: format!(
                        "Known malicious or test signature matched: {}",
                        detection.description
                    ),
                    score: 30,
                }],
                errors: Vec::new(),
            };
        }

        let heuristics = score_file(path, &file_head);
        let score = total_score(&heuristics);
        let verdict = match score {
            0..=29 => ScanVerdict::Clean,
            30..=59 => ScanVerdict::Suspicious,
            _ => ScanVerdict::Malicious,
        };

        ScanResult {
            path: path.to_path_buf(),
            verdict,
            detection_name: detection_name_for_heuristics(verdict, score),
            confidence_score: score,
            matched_rule: heuristics.first().map(|finding| finding.rule_id.clone()),
            sha256: Some(sha256),
            file_metadata: Some(file_metadata),
            scan_duration_ms: started.elapsed().as_millis(),
            heuristics,
            errors: Vec::new(),
        }
    }
}

fn build_file_metadata(path: &Path, metadata: &fs::Metadata) -> FileMetadata {
    FileMetadata {
        size_bytes: metadata.len(),
        extension: path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase()),
        mime_guess: mime_guess::from_path(path).first_raw().map(str::to_string),
        modified_unix_seconds: metadata
            .modified()
            .ok()
            .and_then(system_time_to_unix_seconds),
    }
}

fn system_time_to_unix_seconds(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn read_file_head(path: &Path, size: usize) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0_u8; size];
    let read = file.read(&mut buffer)?;
    buffer.truncate(read);
    Ok(buffer)
}

fn detection_name_for_heuristics(verdict: ScanVerdict, score: u8) -> Option<String> {
    match verdict {
        ScanVerdict::Suspicious => Some(format!("Suspicious.File.Heuristic.{score}")),
        ScanVerdict::Malicious => Some(format!("Malicious.File.Heuristic.{score}")),
        _ => None,
    }
}

fn error_result(path: PathBuf, error: String) -> ScanResult {
    error_result_with_duration(path, error, 0)
}

fn error_result_with_duration(path: PathBuf, error: String, duration_ms: u128) -> ScanResult {
    ScanResult {
        path,
        verdict: ScanVerdict::Error,
        detection_name: None,
        confidence_score: 0,
        matched_rule: None,
        sha256: None,
        file_metadata: None,
        scan_duration_ms: duration_ms,
        heuristics: Vec::new(),
        errors: vec![error],
    }
}
