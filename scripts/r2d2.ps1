$ErrorActionPreference = "Stop"

cargo +nightly build --release --out-dir=.\target\publish -Z unstable-options

& "$PSScriptRoot\start-postgres.ps1"

.\target\publish\r2d2.exe
