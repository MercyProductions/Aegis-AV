# Unified Event Bus

The event bus is the backbone for logs, UI updates, automation, notifications, analytics, and enterprise reporting.

## Event Types

- `ThreatDetected`
- `ScanStarted`
- `ScanFinished`
- `BehaviorTriggered`
- `ConnectionBlocked`
- `FileQuarantined`
- `PolicyChanged`
- `UpdateInstalled`
- `NotificationRaised`
- `AutomationExecuted`
- `ServiceRecovered`

## Event Contract

Each event includes an id, timestamp, kind, source module, severity, summary, and string metadata. Metadata stays small and avoids file contents or sensitive personal data.

## Routing

- Logs receive all events.
- UI receives current status and recent events.
- Automation receives policy-eligible events.
- Enterprise reporting receives summaries only when enabled.
- Notifications receive user-visible events after profile and mode filtering.
