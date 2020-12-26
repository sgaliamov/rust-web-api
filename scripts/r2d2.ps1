$ErrorActionPreference = "Stop"

& "$PSScriptRoot\start-postgres.ps1"

cargo +nightly build --release --out-dir=.\target\publish -Z unstable-options

.\target\publish\r2d2.exe
