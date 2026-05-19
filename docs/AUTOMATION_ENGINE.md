# Workflow Automation Engine

Aegis automation is safe, visible, and policy controlled. It coordinates responses but does not introduce hidden control paths.

## Example Workflows

```txt
If ransomware score > 80:
  isolate process pending confirmation
  quarantine file if policy allows
  create incident report
  notify user

If unknown executable detected:
  upload metadata only if cloud reputation is enabled
  request reputation
  prompt user
```

## Safety Rules

- No destructive action should run without policy support and event logging.
- Isolation actions are pending confirmation unless an explicit managed policy allows them.
- Cloud requests upload metadata only by default.
- Every workflow emits `AutomationExecuted`.
- Users can disable workflows outside managed enterprise mode.
