# Aegis Backend API

Planned FastAPI service for defensive administration tasks:

- signature metadata delivery
- telemetry ingestion
- update manifests
- fleet dashboard APIs

Local development will use SQLite first. PostgreSQL is planned for cloud deployment.

## Run Locally

```powershell
python -m venv .venv
.\.venv\Scripts\pip install -r requirements.txt
.\.venv\Scripts\uvicorn app.main:app --reload --host 127.0.0.1 --port 8088
```

The API is local-development only at this stage and exposes transparent admin workflows: device health, policy assignment, remote scan requests, and quarantine summaries.
