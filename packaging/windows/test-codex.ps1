# Compile library tests together so their shared dependency features stay consistent.
param()
$ErrorActionPreference = 'Stop'
$packages = @('craft-anything-ai', 'craft-anything-setup', 'craft-anything-engine', 'craft-anything-mcp')
$packageArgs = @()
foreach ($package in $packages) { $packageArgs += @('-p', $package) }
$messages = & cargo test --locked --release --target x86_64-pc-windows-msvc @packageArgs --lib --no-run --message-format=json
if ($LASTEXITCODE -ne 0) { throw 'Compiling regression tests failed' }
$artifacts = @($messages | ForEach-Object { $_ | ConvertFrom-Json } | Where-Object {
  $_.reason -eq 'compiler-artifact' -and $_.profile.test -and $_.executable
})
foreach ($package in $packages) {
  $targetName = $package -replace '-', '_'
  $matching = @($artifacts | Where-Object { $_.target.name -eq $targetName })
  if ($matching.Count -ne 1) { throw "Expected exactly one test executable for $package" }
  $artifact = $matching[0]
  $arguments = @()
  if ($package -eq 'craft-anything-engine') { $arguments = @('tools::apple_runtime', '--test-threads=1') }
  Write-Output "::group::$package regressions"
  # Preserve Cargo's per-package working directory while running its reported executable.
  Push-Location (Split-Path -Parent $artifact.manifest_path)
  try {
    & $artifact.executable @arguments
    if ($LASTEXITCODE -ne 0) { throw "$package tests failed" }
  } finally {
    Pop-Location
    Write-Output '::endgroup::'
  }
}
