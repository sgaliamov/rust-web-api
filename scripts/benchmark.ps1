$ErrorActionPreference = "Stop"

dotnet build .\dotnet\NotesApi\Benchmark\Benchmark.csproj -c release -o .\target\benchmark

.\target\benchmark\benchmark.exe
