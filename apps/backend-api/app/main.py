from enum import Enum
from typing import Literal

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field


class PolicyProfile(str, Enum):
    balanced = "balanced"
    strict = "strict"
    performance = "performance"
    silent_gaming_mode = "silent_gaming_mode"
    enterprise_managed = "enterprise_managed"


class DeviceRecord(BaseModel):
    device_id: str
    device_name: str
    health_score: int = Field(ge=0, le=100)
    protection_status: str
    assigned_policy: PolicyProfile
    signature_version: str
    last_seen: str
    threats_blocked: int = Field(ge=0)
    quarantine_count: int = Field(ge=0)


class RemoteScanRequest(BaseModel):
    request_id: str
    device_id: str
    profile: Literal["quick", "full", "deep", "custom"]
    requested_by: str
    created_at: str
    requires_user_visibility: bool = True


class PolicyAssignment(BaseModel):
    policy: PolicyProfile


app = FastAPI(
    title="Aegis Enterprise Admin API",
    version="0.1.0",
    description="Optional transparent admin API for device health, policies, and remote scan requests.",
)

devices: dict[str, DeviceRecord] = {
    "dev-001": DeviceRecord(
        device_id="dev-001",
        device_name="GABRI-WORKSTATION",
        health_score=96,
        protection_status="protected",
        assigned_policy=PolicyProfile.balanced,
        signature_version="2026.05.18.1",
        last_seen="2026-05-18T22:20:00Z",
        threats_blocked=2,
        quarantine_count=7,
    )
}
remote_scan_requests: list[RemoteScanRequest] = []


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.get("/devices", response_model=list[DeviceRecord])
def list_devices() -> list[DeviceRecord]:
    return list(devices.values())


@app.get("/devices/{device_id}", response_model=DeviceRecord)
def get_device(device_id: str) -> DeviceRecord:
    if device_id not in devices:
        raise HTTPException(status_code=404, detail="device not found")
    return devices[device_id]


@app.put("/devices/{device_id}/policy", response_model=DeviceRecord)
def assign_policy(device_id: str, assignment: PolicyAssignment) -> DeviceRecord:
    if device_id not in devices:
        raise HTTPException(status_code=404, detail="device not found")
    current = devices[device_id]
    updated = current.model_copy(update={"assigned_policy": assignment.policy})
    devices[device_id] = updated
    return updated


@app.post("/remote-scans", response_model=RemoteScanRequest)
def request_remote_scan(request: RemoteScanRequest) -> RemoteScanRequest:
    if request.device_id not in devices:
        raise HTTPException(status_code=404, detail="device not found")
    remote_scan_requests.append(request)
    return request


@app.get("/remote-scans", response_model=list[RemoteScanRequest])
def list_remote_scans() -> list[RemoteScanRequest]:
    return remote_scan_requests


@app.get("/quarantine-summary")
def quarantine_summary() -> dict[str, int]:
    return {
        "devices_with_quarantine": sum(1 for device in devices.values() if device.quarantine_count > 0),
        "total_quarantined": sum(device.quarantine_count for device in devices.values()),
    }
