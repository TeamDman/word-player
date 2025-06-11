param(
    [switch]$release
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$env:BEVY_ASSET_PATH = Join-Path $scriptDir 'assets'

if ($release) {
    cargo build --release -p urine
    $targetExe = Join-Path $scriptDir '../../target/release/urine.exe'
} else {
    cargo build -p urine
    $targetExe = Join-Path $scriptDir '../../target/debug/urine.exe'
}

$destDir = Join-Path $scriptDir '../../urine.exe'
Copy-Item $targetExe $destDir -Force