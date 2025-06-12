$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$image = "G:\Programming\Repos\word-player\captures\resized\arnw_10pct.png"
$image = "G:\Programming\Repos\word-player\captures\resized\naip_10pct.png"
cargo run -p piss -- $image
