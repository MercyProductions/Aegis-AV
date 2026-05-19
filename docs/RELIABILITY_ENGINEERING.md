# Reliability Engineering

Aegis treats reliability as a product feature. Defensive software must recover cleanly, explain what happened, and avoid creating mystery states.

## Stability Signals

- `CrashDetected`
- `WatchdogMissedHeartbeat`
- `CorruptionDetected`
- `UpdateFailed`
- `MemoryGrowth`
- `DeadlockSuspected`
- `LongRuntimeStressFailure`
- `ServiceStopped`

## Recovery Actions

- Capture a crash report.
- Restart the affected service.
- Enter safe mode with minimal trusted components.
- Roll back to the last verified update snapshot.
- Repair corrupted local state.
- Throttle a module that is causing resource pressure.
- Run targeted stress tests.

## Rules

- Recovery must be visible in the event log.
- Update rollback must use previously verified packages only.
- Safe mode must not hide from the user.
- Self-protection must warn and recover, not evade or abuse persistence.
