#!/usr/bin/env pwsh
Write-Host "Running pre-commit checks: cargo fmt, clippy"

$root = git rev-parse --show-toplevel 2>$null
if (-not $?) { $root = '.' }
Set-Location $root

Write-Host '-> Running cargo fmt --all'
cargo fmt --all
if ($LASTEXITCODE -ne 0) {
  Write-Error "cargo fmt failed ($LASTEXITCODE). Aborting commit."
  exit $LASTEXITCODE
}

Write-Host '-> Attempting clippy autofix with nightly (if available)'
if (Get-Command rustup -ErrorAction SilentlyContinue) {
  $toolchains = rustup toolchain list
  if ($toolchains -match 'nightly') {
    Write-Host 'nightly toolchain detected — running clippy --fix'
    rustup run nightly cargo clippy --fix -Z unstable-options | Out-Null
  } else {
    Write-Host 'nightly toolchain not found — skipping clippy --fix'
  }
} else {
  Write-Host 'rustup not found — skipping clippy --fix'
}

Write-Host '-> Running cargo clippy (deny warnings)'
cargo clippy -- -D warnings
if ($LASTEXITCODE -ne 0) {
  Write-Error "cargo clippy found issues ($LASTEXITCODE). Fix or run 'rustup run nightly cargo clippy --fix -Z unstable-options' then commit again."
  exit $LASTEXITCODE
}

Write-Host 'Pre-commit checks passed'
exit 0
