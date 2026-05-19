# Cross-Platform Architecture

Aegis is Windows-first, with core contracts shaped so Linux and macOS adapters can be added later.

## Layers

- Platform layer: filesystem, process, service, notifications.
- Core engine: scanner, signatures, policies, events, incidents.
- UI layer: Electron shell and platform integrations.
- Driver layer: future signed platform-specific work.
- Update layer: signed packages, rollback, channels, offline updates.
- Event layer: shared telemetry and automation backbone.
- Plugin layer: signed, sandboxed, permissioned modules.

## Portability Rule

Anything that touches OS-specific APIs must live behind a platform adapter. Core models should stay deterministic, testable, and serializable.
