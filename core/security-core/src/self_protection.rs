use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustedComponent {
    pub name: String,
    pub path: PathBuf,
    pub expected_sha256: String,
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelfProtectionManifest {
    pub manifest_version: u32,
    pub generated_at: String,
    pub components: Vec<TrustedComponent>,
    pub signature_database_hash: Option<String>,
    pub config_hash: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SelfProtectionStatus {
    Healthy,
    Missing,
    Corrupted,
    Unchecked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelfProtectionCheck {
    pub component_name: String,
    pub path: PathBuf,
    pub status: SelfProtectionStatus,
    pub expected_sha256: Option<String>,
    pub actual_sha256: Option<String>,
    pub critical: bool,
    pub message: String,
}

impl SelfProtectionManifest {
    pub fn verify_components(&self, install_root: &Path) -> Vec<SelfProtectionCheck> {
        self.components
            .iter()
            .map(|component| verify_component(install_root, component))
            .collect()
    }
}

fn verify_component(install_root: &Path, component: &TrustedComponent) -> SelfProtectionCheck {
    let path = if component.path.is_absolute() {
        component.path.clone()
    } else {
        install_root.join(&component.path)
    };

    if !path.exists() {
        return SelfProtectionCheck {
            component_name: component.name.clone(),
            path,
            status: SelfProtectionStatus::Missing,
            expected_sha256: Some(component.expected_sha256.clone()),
            actual_sha256: None,
            critical: component.critical,
            message: "component is missing".to_string(),
        };
    }

    match sha256_file(&path) {
        Ok(actual) if actual == component.expected_sha256.to_ascii_lowercase() => {
            SelfProtectionCheck {
                component_name: component.name.clone(),
                path,
                status: SelfProtectionStatus::Healthy,
                expected_sha256: Some(component.expected_sha256.clone()),
                actual_sha256: Some(actual),
                critical: component.critical,
                message: "component hash verified".to_string(),
            }
        }
        Ok(actual) => SelfProtectionCheck {
            component_name: component.name.clone(),
            path,
            status: SelfProtectionStatus::Corrupted,
            expected_sha256: Some(component.expected_sha256.clone()),
            actual_sha256: Some(actual),
            critical: component.critical,
            message: "component hash mismatch".to_string(),
        },
        Err(error) => SelfProtectionCheck {
            component_name: component.name.clone(),
            path,
            status: SelfProtectionStatus::Unchecked,
            expected_sha256: Some(component.expected_sha256.clone()),
            actual_sha256: None,
            critical: component.critical,
            message: error.to_string(),
        },
    }
}

fn sha256_file(path: &Path) -> std::io::Result<String> {
    let bytes = fs::read(path)?;
    let digest = Sha256::digest(bytes);
    Ok(hex_lower(&digest))
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_component_hash() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("component.bin");
        fs::write(&path, b"trusted").expect("write");
        let expected_sha256 = sha256_file(&path).expect("hash");
        let manifest = SelfProtectionManifest {
            manifest_version: 1,
            generated_at: "2026-05-18T12:00:00Z".to_string(),
            components: vec![TrustedComponent {
                name: "component".to_string(),
                path: PathBuf::from("component.bin"),
                expected_sha256,
                critical: true,
            }],
            signature_database_hash: None,
            config_hash: None,
        };

        let checks = manifest.verify_components(dir.path());

        assert_eq!(checks[0].status, SelfProtectionStatus::Healthy);
    }
}
