# Endpoint Platform Architecture

## Local Components

- `aegis-scanner`: file and folder scanner with profile-aware target planning.
- `aegis-security-core`: shared behavior, process tree, ransomware, reputation, logging, performance, and update trust models.
- `aegis-quarantine`: quarantine metadata, duplicate prevention, restore validation, and incident report contracts.
- `aegis-agent`: future Windows service host for filesystem events, process snapshots, and local UI IPC.
- `apps/desktop-ui`: Electron + React command center.

## Behavior Response Model

Behavioral monitoring scores events and returns a recommended response. It does not blindly terminate processes or delete files.

| Risk | Response |
| --- | --- |
| Low | Log only |
| Medium | Notify user |
| High | Pause or quarantine pending confirmation |
| Critical | Auto-quarantine only when explicitly enabled |

## Process Data

The process tree model captures process name, PID, parent PID, path, command line, signature status, hash, start time, and risk score. The first implementation stores and renders relationships; native Windows collection belongs in the service agent phase.

## Ransomware Guard

Protected-folder events are scored for rename bursts, extension changes, high-entropy write hints, mass writes, and deletion bursts. Strong responses remain policy controlled.
