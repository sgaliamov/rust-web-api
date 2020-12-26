$ErrorActionPreference = "Stop"

& "$PSScriptRoot\start-postgres.ps1"

dotnet build .\dotnet\NotesApi\Benchmark\Benchmark.csproj -c release -o .\target\benchmark

.\target\benchmark\benchmark.exe
