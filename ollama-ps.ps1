while ($true) {
    $out = ollama ps
    Clear-Host
    $out | Write-Host
    Start-Sleep -Seconds 1
}