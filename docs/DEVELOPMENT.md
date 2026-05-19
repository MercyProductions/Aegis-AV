# Development Notes

## Stack

- Core scanner and future service agent: Rust
- Desktop UI: Electron + React + TypeScript, planned
- Backend API: FastAPI, planned
- Local development data: SQLite, planned
- Signature delivery: JSON first, SQLite/custom binary later

## Workspace

The Rust workspace currently includes:

- `core/scanner`: scanner engine and CLI
- `core/agent`: future Windows service agent placeholder
- `core/updater`: future update verifier placeholder
- `core/quarantine`: quarantine data model placeholder

## Defensive Testing

Do not commit live malware samples. EICAR is the only antivirus test signature included in the default hash database.

The repo intentionally avoids storing the raw EICAR string as a fixture because host antivirus tools often quarantine files containing it. Tests build the string from chunks when needed.
