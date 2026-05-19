# Local Guard Agent

The local guard agent is a visible foreground process that keeps Aegis scanning repeatedly until you stop it. It is not hidden, does not install persistence, and does not start automatically.

## Build The EXE

```powershell
cargo build --release -p aegis-agent -p aegis-scanner
```

The agent EXE is created at:

```txt
target/release/aegis-agent.exe
```

The scanner EXE is created at:

```txt
target/release/aegis-scanner.exe
```

## Launch The GUI

The desktop UI now has a `Device Control` page with buttons for:

- Start Guard
- Arm
- Disarm
- Stop Guard
- Refresh Status

The simplest local launch is:

```txt
AegisAV.exe
```

Double-click that file from the project root.

Rebuild the portable EXE with:

```powershell
cd apps\desktop-ui
npm run package:win
cd ..\..
Copy-Item apps\desktop-ui\release-desktop\AegisAV-0.1.0-x64.exe .\AegisAV.exe -Force
```

For development only, you can still launch from source:

```powershell
cargo build --release -p aegis-agent -p aegis-scanner
cd apps\desktop-ui
npm install
npm run desktop
```

The browser-based dev UI at `http://127.0.0.1:5173` can show the interface, but native guard controls require the Electron desktop app because browsers cannot safely start local EXEs.

## Arm Protection

```powershell
target\release\aegis-agent.exe arm
```

This watches Downloads, Desktop, Documents, and Temp by default.

Use custom folders:

```powershell
target\release\aegis-agent.exe arm --path "C:\Users\gabri\Downloads" --path "C:\Users\gabri\Desktop" --interval-seconds 30
```

## Run The Guard

```powershell
target\release\aegis-agent.exe run
```

Leave that window open. Stop it with `Ctrl+C`.

Arm and run in one command:

```powershell
target\release\aegis-agent.exe run --arm --interval-seconds 30
```

## Disarm Protection

Open another PowerShell window and run:

```powershell
target\release\aegis-agent.exe disarm
```

The running guard process will notice the state change and pause scanning.

## Check Status

```powershell
target\release\aegis-agent.exe status
```

## Local State And Logs

The guard stores state in:

```txt
C:\ProgramData\AegisAV\agent-state.json
```

Logs are written to:

```txt
C:\ProgramData\AegisAV\Logs\agent-events.jsonl
```

## Current Limits

- This is a user-started foreground guard, not an installed Windows service yet.
- It scans on an interval instead of receiving kernel-level filesystem events.
- It alerts and logs detections; automatic quarantine remains a later, policy-controlled feature.
