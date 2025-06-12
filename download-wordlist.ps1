$url = "https://www.freescrabbledictionary.com/enable/download/enable.txt"
$destination = "enable.txt"
try {
    # Download the file
    Invoke-WebRequest -Uri $url -OutFile $destination -ErrorAction Stop
    Write-Host "Download completed successfully."
} catch {
    Write-Host "An error occurred while downloading the file: $_"
}