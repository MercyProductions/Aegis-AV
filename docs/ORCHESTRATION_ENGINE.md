# Intelligent Orchestration

The orchestration engine upgrades automation into reviewable workflows.

## Example Plans

```txt
If suspicious process launches:
  isolate process pending review
  capture metadata
  scan related files
  generate incident
  notify user

If ransomware behavior is detected:
  pause process according to policy
  protect folders
  snapshot affected files if enabled
  launch recovery workflow
  generate incident
```

## Safety Rules

- Actions are reviewable by default.
- Process isolation requires user or managed-policy authority.
- Every step emits an event.
- Reports explain what happened and why.
- No workflow hides from the user.
