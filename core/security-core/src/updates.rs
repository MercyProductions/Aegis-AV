use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedUpdateManifest {
    pub channel: ReleaseChannel,
    pub version: String,
    pub package_url: String,
    pub package_sha256: String,
    pub manifest_signature: Option<String>,
    pub minimum_app_version: String,
    pub rollback_counter: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannel {
    Stable,
    Beta,
    Dev,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateVerification {
    pub hash_matches: bool,
    pub has_manifest_signature: bool,
    pub rollback_allowed: bool,
    pub can_apply: bool,
    pub notes: Vec<String>,
}

pub fn verify_update_package(
    manifest: &SignedUpdateManifest,
    package_bytes: &[u8],
    current_rollback_counter: u64,
) -> UpdateVerification {
    let hash = Sha256::digest(package_bytes);
    let actual = hex_lower(&hash);
    let hash_matches = actual == manifest.package_sha256.to_ascii_lowercase();
    let has_manifest_signature = manifest
        .manifest_signature
        .as_deref()
        .is_some_and(|sig| !sig.is_empty());
    let rollback_allowed = manifest.rollback_counter >= current_rollback_counter;
    let can_apply = hash_matches && has_manifest_signature && rollback_allowed;
    let mut notes = Vec::new();

    if !hash_matches {
        notes.push("package SHA256 does not match manifest".to_string());
    }
    if !has_manifest_signature {
        notes.push("manifest is unsigned; automatic apply is not allowed".to_string());
    }
    if !rollback_allowed {
        notes.push("rollback counter is older than installed state".to_string());
    }

    UpdateVerification {
        hash_matches,
        has_manifest_signature,
        rollback_allowed,
        can_apply,
        notes,
    }
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
    fn unsigned_manifest_cannot_apply_even_with_matching_hash() {
        let bytes = b"safe signature pack";
        let manifest = SignedUpdateManifest {
            channel: ReleaseChannel::Stable,
            version: "2026.05.18.1".to_string(),
            package_url: "https://updates.example/signatures.zip".to_string(),
            package_sha256: hex_lower(&Sha256::digest(bytes)),
            manifest_signature: None,
            minimum_app_version: "1.0.0".to_string(),
            rollback_counter: 1,
        };

        let verification = verify_update_package(&manifest, bytes, 0);

        assert!(verification.hash_matches);
        assert!(!verification.can_apply);
    }
}
