$srcDir = "captures"
$dstDir = "captures\resized"

if (-not (Test-Path $dstDir)) {
    New-Item -ItemType Directory -Path $dstDir | Out-Null
}

$sizes = @(0.1, 0.2, 0.3, 0.4, 0.5)

Get-ChildItem -Path "$srcDir\*" -File -Include *.jpg,*.jpeg,*.png,*.bmp,*.gif | ForEach-Object {
    Write-Host "Resizing image: $($_.Name)"
    $img = $_
    foreach ($scale in $sizes) {
        $scalePercent = [math]::Round($scale * 100)
        $newName = "{0}_{1}pct{2}" -f $img.BaseName, $scalePercent, $img.Extension
        $dstPath = Join-Path $dstDir $newName
        magick "$($img.FullName)" -resize "$($scalePercent)%" "$dstPath"
    }
}