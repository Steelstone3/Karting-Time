# Define the GitHub API URL for the latest release
$repo = "Steelstone3/Karting-Time"
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"

# Set headers to avoid GitHub API rate limiting
$headers = @{
    "User-Agent" = "WindowsPowerShell"
}

# Fetch the latest release info
$response = Invoke-RestMethod -Uri $apiUrl -Headers $headers

# Get the first asset's download URL and name
$asset = $response.assets | Where-Object { $_.name -like "*.zip" } | Select-Object -First 1
$downloadUrl = $asset.browser_download_url
$fileName = $asset.name

# Download the asset
Invoke-WebRequest -Uri $downloadUrl -OutFile $fileName

# Extract the ZIP file
$extractPath = "$PWD\Extracted"
Expand-Archive -Path $fileName -DestinationPath $extractPath -Force

Write-Host "Downloaded and extracted $fileName to $extractPath"



# Ensure the target directory exists
$targetDir = "$PWD\target\releases"
if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force
}

# Find and move karting-time.exe
$extractedExe = Get-ChildItem -Path $extractPath -Recurse -Filter "karting-time.exe" | Select-Object -First 1
if ($extractedExe) {
    Move-Item -Path $extractedExe.FullName -Destination $targetDir -Force
    Write-Host "Moved karting-time.exe to $targetDir"
} else {
    Write-Host "karting-time.exe not found in extracted files."
}



# Remove the ZIP file
Remove-Item -Path $fileName -Force
Write-Host "Removed ZIP file: $fileName"



# Run the executable
Start-Process -FilePath "$targetDir\karting-time.exe"

