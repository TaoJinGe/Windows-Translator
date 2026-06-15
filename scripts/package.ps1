$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
$targetDir = Join-Path $root "src-tauri\target-package"
$releaseDir = Join-Path $root "release"
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"

if (Test-Path $cargoBin) {
  $env:PATH = "$cargoBin;$env:PATH"
}

$env:CARGO_TARGET_DIR = $targetDir

Push-Location $root
try {
  npm.cmd run tauri -- build
  if ($LASTEXITCODE -ne 0) {
    throw "Tauri build failed with exit code $LASTEXITCODE"
  }

  New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

  $exe = Join-Path $targetDir "release\windows-translator.exe"
  $msi = Get-ChildItem -Path (Join-Path $targetDir "release\bundle\msi") -Filter "*.msi" |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

  Copy-Item -LiteralPath $exe -Destination (Join-Path $releaseDir "windows-translator.exe") -Force
  Copy-Item -LiteralPath $msi.FullName -Destination (Join-Path $releaseDir $msi.Name) -Force

  Write-Host "Release artifacts:"
  Write-Host " - release\windows-translator.exe"
  Write-Host " - release\$($msi.Name)"
}
finally {
  Pop-Location
}
