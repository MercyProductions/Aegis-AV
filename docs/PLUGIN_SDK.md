# Plugin SDK

Plugins are planned extension points for Network, Diagnostics, Developer Tools, Backup, Enterprise, and AI Analysis features.

## Manifest Requirements

- `plugin_id`
- `name`
- `version`
- `signed_manifest`
- `enabled`
- `sandboxed`
- `permissions`

Production plugins must be sandboxed and signed. Unsigned plugins can exist in development but should surface warnings and remain easy to disable.

## Permissions

- `read_events`
- `read_diagnostics`
- `read_network_metadata`
- `request_scan`
- `write_reports`
- `enterprise_admin`

Plugins must never receive broad filesystem or network authority by default.
