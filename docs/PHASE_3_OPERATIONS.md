# Phase 3 Operations

## Security Center

The Security Center is the main home screen for protection score, active shields, last scan, blocked threats, ransomware status, update health, device health, recent events, and recommended actions.

## Incident Reports

`aegis-security-core` can export incident reports as JSON, HTML, and a simple PDF report. Reports include detection name, severity, file path, SHA256, optional process tree, event timeline, matched signatures and rules, heuristic reasons, user action, and recommended next steps.

## Policy Engine

The policy engine currently supports:

- Balanced
- Strict
- Performance
- Silent / Gaming Mode
- Enterprise Managed

Each policy controls quarantine behavior, notifications, scan priority, CPU limit, realtime scan depth, behavior sensitivity, ransomware protection, and update behavior.

## Self-Protection

Self-protection is transparent and defensive. It verifies component hashes, signature database integrity, config integrity, service health signals, and quarantine folder protection. It does not use stealth, evasion, hidden persistence, or security bypasses.

## Cloud Sync

Cloud sync is opt-in. The default payload contract rejects personal file contents and only syncs license status, device name, protection status, scan summaries, policy settings, and false positive report metadata.

## Enterprise Console

The optional FastAPI admin scaffold supports device listing, device health, policy assignment, remote scan requests, and quarantine summaries.
