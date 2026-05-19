use aegis_scanner::hashing::sha256_bytes;
use aegis_scanner::{
    resolve_profile_targets, ScanEngine, ScanOptions, ScanProfile, ScanVerdict, SignatureDb,
    SignatureRecord,
};
use std::fs;

fn eicar_bytes() -> Vec<u8> {
    [
        "X5O!P%@AP[4\\PZX54(P^)7CC)7}",
        "$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!",
        "$H+H*",
    ]
    .concat()
    .into_bytes()
}

#[test]
fn canonical_eicar_hash_matches_default_signature() {
    let hash = sha256_bytes(&eicar_bytes());

    assert_eq!(
        hash,
        "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f"
    );
}

#[test]
fn detects_hash_signature_for_harmless_test_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("harmless.txt");
    fs::write(&path, b"harmless aegis test content").expect("write test file");
    let hash = sha256_bytes(b"harmless aegis test content");

    let db = SignatureDb::from_records(vec![SignatureRecord {
        id: "AEGIS-TEST-HARMLESS".to_string(),
        name: "Test.Harmless.Hash".to_string(),
        signature_type: "test".to_string(),
        severity: "medium".to_string(),
        description: "Harmless test hash used for scanner verification.".to_string(),
        recommended_action: "quarantine".to_string(),
        sha256: hash,
    }]);
    let engine = ScanEngine::new(db, ScanOptions::default());
    let result = engine.scan_file(&path);

    assert_eq!(result.verdict, ScanVerdict::Malicious);
    assert_eq!(result.detection_name.as_deref(), Some("Test.Harmless.Hash"));
    assert_eq!(result.matched_rule.as_deref(), Some("AEGIS-TEST-HARMLESS"));
}

#[test]
#[ignore = "writes the raw EICAR test string; run intentionally in a controlled test folder"]
fn detects_eicar_file_when_intentionally_written() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("eicar.com.txt");
    fs::write(&path, eicar_bytes()).expect("write eicar test file");

    let db = SignatureDb::from_records(vec![SignatureRecord {
        id: "AEGIS-TEST-0001".to_string(),
        name: "Test.EICAR.Signature".to_string(),
        signature_type: "test".to_string(),
        severity: "high".to_string(),
        description: "Safe antivirus test signature.".to_string(),
        recommended_action: "quarantine".to_string(),
        sha256: "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f".to_string(),
    }]);
    let engine = ScanEngine::new(db, ScanOptions::default());
    let result = engine.scan_file(&path);

    assert_eq!(result.verdict, ScanVerdict::Malicious);
    assert_eq!(
        result.detection_name.as_deref(),
        Some("Test.EICAR.Signature")
    );
}

#[test]
fn custom_profile_uses_requested_target() {
    let plan = resolve_profile_targets(ScanProfile::Custom, Some("C:/Example".into()));

    assert_eq!(plan.targets, vec![std::path::PathBuf::from("C:/Example")]);
    assert_eq!(plan.label, "Custom Scan");
}
