# Production Readiness Checklist

## Permissions

- Service runs with the least privileges needed for scanning and quarantine.
- UI communicates with the service through authenticated local-only IPC.
- Destructive actions require explicit user confirmation unless policy enables automatic quarantine.

## Detection

- Hash signatures include test and production namespaces.
- YARA rules are validated before loading.
- Heuristic rules explain every score contribution.
- Reputation data is versioned and auditable.

## Quarantine

- Files are moved to a protected ProgramData folder.
- Original path, hash, detection, timestamp, preview metadata, and restore policy are recorded.
- Restore validates destination and prevents overwrite by default.
- Incident reports can be exported without file contents.

## Updates

- Manifests are signed.
- Signature packs are signed.
- Installer and binaries are code signed.
- Rollback counters prevent downgrade attacks.
- Failed updates roll back to the last known good state.

## Privacy

- Logs do not include file contents.
- Telemetry is opt-in for consumer mode.
- Enterprise mode displays active policy and reporting status.

## Reliability

- Agent crash recovery preserves scan and quarantine state.
- Hash cache entries are invalidated by size and modified time.
- Battery saver lowers scan priority.
- Large files follow explicit handling rules.

## Phase 3 QA Targets

- 100k+ file scan fixture.
- Large nested folders.
- Low-end CPU and battery saver runs.
- Permission denied folders.
- Corrupted files.
- Update verification failures.
- Service crash and UI reconnect.
- Quarantine restore validation.
- Offline mode.
- Admin API policy assignment.
