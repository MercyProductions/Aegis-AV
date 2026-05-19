# Plugin Marketplace

The plugin marketplace is a controlled ecosystem for optional Aegis capabilities.

## Requirements

- Signed plugin manifests.
- Explicit permissions.
- Sandbox isolation.
- Versioned releases.
- Disable and uninstall controls.
- Local audit trail for plugin actions.

## Permission Examples

- Read event summaries.
- Read diagnostics metadata.
- Read network metadata.
- Request a scan.
- Write reports.
- Enterprise admin actions.

Plugins must never bypass Aegis policy, hide from the user, or disable other security tools.
