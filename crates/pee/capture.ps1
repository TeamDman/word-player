$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

Write-Host "Getting coordinates to capture"
$xywh = Invoke-Expression "$(Join-Path $scriptDir "..\..\urine.exe") --prompt 'Identify the letter grid' | Out-String"
if ($xywh -eq "") {
    Write-Host "Capture cancelled by user"
    exit 1
}

$xywh = $xywh.Split(' ')

Write-Host "Waiting for capture overlay to finish closing"
Start-Sleep -Milliseconds 100 # Wait for the window to finish closing lol

Write-Host "Capturing the screen"
Invoke-Expression "$(Join-Path $scriptDir '..\..\pee.exe') $xywh"