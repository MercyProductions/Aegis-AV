$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $RepoRoot

try {
    cargo build --release -p aegis-agent -p aegis-scanner

    Push-Location "apps\desktop-ui"
    try {
        if (-not (Test-Path "node_modules")) {
            npm install
        }

        npm run build
        npm run electron:build
        npx electron .
    }
    finally {
        Pop-Location
    }
}
finally {
    Pop-Location
}
