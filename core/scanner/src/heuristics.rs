use crate::types::HeuristicFinding;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const EXECUTABLE_EXTENSIONS: &[&str] =
    &["exe", "dll", "scr", "com", "bat", "cmd", "ps1", "js", "vbs"];
const DOCUMENT_EXTENSIONS: &[&str] = &[
    "pdf", "doc", "docx", "xls", "xlsx", "jpg", "jpeg", "png", "txt",
];
const SCRIPT_EXTENSIONS: &[&str] = &["ps1", "js", "vbs", "bat", "cmd"];

pub fn score_file(path: &Path, file_head: &[u8]) -> Vec<HeuristicFinding> {
    let mut findings = Vec::new();
    let extension = normalized_extension(path);

    if has_double_extension(path) {
        findings.push(HeuristicFinding {
            rule_id: "HEUR-DOUBLE-EXTENSION".to_string(),
            description:
                "File name uses a document-like extension followed by an executable extension."
                    .to_string(),
            score: 15,
        });
    }

    if is_executable_extension(extension.as_deref()) && is_in_temp_path(path) {
        findings.push(HeuristicFinding {
            rule_id: "HEUR-EXECUTABLE-IN-TEMP".to_string(),
            description: "Executable or script file is located in a temporary folder.".to_string(),
            score: 20,
        });
    }

    if has_mz_header(file_head)
        && !matches!(
            extension.as_deref(),
            Some("exe" | "dll" | "scr" | "com" | "sys")
        )
    {
        findings.push(HeuristicFinding {
            rule_id: "HEUR-EXTENSION-MISMATCH".to_string(),
            description:
                "File content looks like a Windows executable but the extension does not match."
                    .to_string(),
            score: 20,
        });
    }

    if is_script_extension(extension.as_deref()) && looks_obfuscated(path) {
        findings.push(HeuristicFinding {
            rule_id: "HEUR-SCRIPT-OBFUSCATION".to_string(),
            description: "Script contains common obfuscation indicators such as encoded commands or dynamic evaluation.".to_string(),
            score: 25,
        });
    }

    findings
}

pub fn total_score(findings: &[HeuristicFinding]) -> u8 {
    findings
        .iter()
        .map(|finding| finding.score as u16)
        .sum::<u16>()
        .min(100) as u8
}

fn normalized_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|value| value.trim_start_matches('.').to_ascii_lowercase())
}

fn is_executable_extension(extension: Option<&str>) -> bool {
    extension
        .map(|ext| EXECUTABLE_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

fn is_script_extension(extension: Option<&str>) -> bool {
    extension
        .map(|ext| SCRIPT_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

fn has_double_extension(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    let parts: Vec<_> = file_name
        .split('.')
        .map(|part| part.to_ascii_lowercase())
        .collect();
    if parts.len() < 3 {
        return false;
    }

    let prior = &parts[parts.len() - 2];
    let last = &parts[parts.len() - 1];
    DOCUMENT_EXTENSIONS.contains(&prior.as_str()) && EXECUTABLE_EXTENSIONS.contains(&last.as_str())
}

fn is_in_temp_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.contains("\\temp\\")
        || lower.contains("/temp/")
        || lower.contains("\\tmp\\")
        || lower.contains("/tmp/")
        || lower.ends_with("\\temp")
        || lower.ends_with("/temp")
}

fn has_mz_header(file_head: &[u8]) -> bool {
    file_head.starts_with(b"MZ")
}

fn looks_obfuscated(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };
    let mut buffer = vec![0_u8; 64 * 1024];
    let read = match file.read(&mut buffer) {
        Ok(read) => read,
        Err(_) => return false,
    };

    let text = String::from_utf8_lossy(&buffer[..read]).to_ascii_lowercase();
    let indicators = [
        "frombase64string",
        "-enc ",
        "-encodedcommand",
        "eval(atob",
        "iex ",
        "invoke-expression",
    ];

    indicators.iter().any(|indicator| text.contains(indicator))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn scores_double_extension() {
        let path = PathBuf::from("invoice.pdf.exe");
        let findings = score_file(&path, b"MZ");

        assert!(findings
            .iter()
            .any(|finding| finding.rule_id == "HEUR-DOUBLE-EXTENSION"));
    }
}
