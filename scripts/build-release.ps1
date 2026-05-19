param(
  [ValidateSet("alpha", "beta", "stable")]
  [string]$Channel = "alpha"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot

Push-Location $Root
try {
  cargo fmt --all -- --check
  cargo clippy --workspace -- -D warnings
  cargo test --workspace

  Push-Location "apps/desktop-ui"
  try {
    npm ci
    npm run build
    npm run electron:build
    npm audit --audit-level=high
  }
  finally {
    Pop-Location
  }

  Write-Host "Release validation passed for $Channel channel."
  Write-Host "Installer signing and artifact upload are intentionally manual until signing keys are configured."
}
finally {
  Pop-Location
}
