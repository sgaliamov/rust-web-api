 & "$PSScriptRoot\remove-postgres.ps1"

docker-compose -f "$PSScriptRoot\postgres-stack.yml" up -d
