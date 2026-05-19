# Modular Architecture

Aegis is organized as independently versioned modules:

- Aegis.Core
- Aegis.Scanner
- Aegis.Realtime
- Aegis.Behavior
- Aegis.Quarantine
- Aegis.Diagnostics
- Aegis.Firewall
- Aegis.Network
- Aegis.ProcessMonitor
- Aegis.Ransomware
- Aegis.Updater
- Aegis.UI
- Aegis.Telemetry
- Aegis.Engine

Each module has a manifest with a module id, display name, semantic version, API version, update channel, dependency list, permissions, and future hot-reload capability. Core and Engine are intentionally locked from hot reload because they coordinate trust boundaries.

## Rules

- Modules must expose typed APIs.
- Modules must be testable independently.
- Module updates must be signed before production enablement.
- Hot reload must validate dependencies and API compatibility.
- Disabled modules remain visible to the user.
