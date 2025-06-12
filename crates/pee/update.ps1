param(
    [switch]$release
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

if ($release) {
    cargo build --release -p pee
    $targetExe = Join-Path $scriptDir '../../target/release/pee.exe'
} else {
    cargo build -p pee
    $targetExe = Join-Path $scriptDir '../../target/debug/pee.exe'
}

$destDir = Join-Path $scriptDir '../../pee.exe'
Copy-Item $targetExe $destDir -Force