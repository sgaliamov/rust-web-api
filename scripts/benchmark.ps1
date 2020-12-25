$ErrorActionPreference = "Stop"

dotnet build .\dotnet\NotesApi\Benchmark\Benchmark.csproj -c release -o .\target\benchmark

& "$PSScriptRoot\start-postgres.ps1"

.\target\benchmark\benchmark.exe
