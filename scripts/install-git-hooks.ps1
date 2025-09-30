param()
$root = git rev-parse --show-toplevel 2>$null
if (-not $?) { Write-Error "Not a git repository or git not found"; exit 1 }

$hooksDir = Join-Path $root '.git\hooks'
$githooksDir = Join-Path $root '.githooks'

Copy-Item -Path (Join-Path $githooksDir 'pre-commit.ps1') -Destination (Join-Path $hooksDir 'pre-commit.ps1') -Force
Copy-Item -Path (Join-Path $githooksDir 'pre-commit') -Destination (Join-Path $hooksDir 'pre-commit') -Force

Write-Host "Installed git hooks to $hooksDir"
