# Aegis AntiVirus

Aegis AntiVirus is a lawful defensive endpoint protection project. The current build focuses on the Phase 1 foundation and a Phase 2 scanner CLI that can safely detect the EICAR antivirus test signature by SHA256 hash.

## Current Status

- Monorepo folder structure is in place.
- Rust workspace is configured for scanner, agent, updater, and quarantine crates.
- Shared security core models are in place for behavior, processes, reputation, ransomware protection, logging, performance, and update trust.
- Phase 3 production operations models are in place for incident reports, policies, self-protection, false positives, threat intelligence, cloud sync, and enterprise administration.
- Phase 4 ecosystem models are in place for modular architecture, layered protection, diagnostics, firewall visibility, sandboxing, explain-only AI, plugins, differential updates, power tools, security scoring, and Aegis integrations.
- Phase 5 maturity models are in place for reliability engineering, unified events, safe automation, notifications, advanced user profiles, threat knowledge, marketplace trust, operations dashboards, cross-platform contracts, brand identity, and long-term platform evolution.
- Phase 6 Aegis OS models are in place for a central AegisCore runtime, live system graph, orchestration, predictive risk, AI operations, observability, workspaces, visual intelligence, distributed architecture, developer APIs, transparency, and premium operating-environment UX.
- `core/scanner` provides a working CLI with:
  - file and folder scanning
  - Quick, Full, Deep, and Custom scan profile planning
  - recursive directory traversal
  - SHA256 hashing
  - JSON hash signature loading
  - EICAR test signature detection
  - safe heuristic scoring
  - large-file skipping
  - permission error handling
  - JSON or console output
- `apps/desktop-ui` contains the first Electron + React + TypeScript command-center UI scaffold.
- `apps/backend-api` contains an optional FastAPI admin console scaffold.
- `core/quarantine` includes restore validation, duplicate prevention, and incident report contracts.

## Run The Scanner

From this directory:

```powershell
cargo run -p aegis-scanner -- scan . --profile custom
```

JSON output:

```powershell
cargo run -p aegis-scanner -- scan . --json
```

Use a custom signature file:

```powershell
cargo run -p aegis-scanner -- scan C:\Path\To\File --signatures core\signatures\hashes.json
```

## Run The Local Guard Agent

Build local EXEs:

```powershell
cargo build --release -p aegis-agent -p aegis-scanner
```

Arm and run visible foreground protection:

```powershell
target\release\aegis-agent.exe run --arm --interval-seconds 30
```

Disarm from another PowerShell window:

```powershell
target\release\aegis-agent.exe disarm
```

See [docs/LOCAL_GUARD_AGENT.md](docs/LOCAL_GUARD_AGENT.md) for status, custom watched folders, and log locations.

## Launch The Desktop Control App

Use the GUI if you do not want to run guard commands manually:

```powershell
.\scripts\launch-aegis-desktop.ps1
```

Open the `Device Control` page to Start Guard, Arm, Disarm, Stop Guard, and refresh status. Native guard controls are available in Electron, not the browser-only dev page.

## Safety Boundaries

This project is defensive and transparent. It does not implement malware execution, stealth, bypasses, credential access, disabling other security tools, unauthorized remote control, or live malware handling.

Use only safe test material such as EICAR, dummy files, and harmless scripts.
