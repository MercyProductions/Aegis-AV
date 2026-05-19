pub use aegis_security_core::updates::{
    verify_update_package, ReleaseChannel, SignedUpdateManifest, UpdateVerification,
};

pub fn component_status() -> &'static str {
    "Aegis update verifier is ready for signed manifest and hash verification workflows."
}
