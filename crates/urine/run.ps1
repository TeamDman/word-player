$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$env:BEVY_ASSET_PATH = Join-Path $scriptDir '../../assets'
cargo run -p urine -- --prompt "Identify the letter grid" --inspector
