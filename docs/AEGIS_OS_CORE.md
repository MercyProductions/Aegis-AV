# Aegis OS Core

`AegisCore` is the central runtime for the Aegis security operating environment.

## Responsibilities

- Module management.
- Event routing.
- Permission enforcement.
- Settings coordination.
- Telemetry controls.
- Automation orchestration.
- Service orchestration.
- UI synchronization.
- Plugin loading.
- Diagnostics coordination.

## Design Rules

- Local-first is the default.
- Cloud sync is optional.
- Every module must declare permissions.
- Every privileged action routes through events.
- UI state mirrors core state instead of inventing its own truth.
