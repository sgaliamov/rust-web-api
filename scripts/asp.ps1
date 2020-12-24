$ErrorActionPreference = "Stop"

& "$PSScriptRoot\start-postgres.ps1"

dotnet build .\dotnet\NotesApi\NotesApi\NotesApi.csproj -c release -o .\target\asp

.\target\asp\notesapi.exe
