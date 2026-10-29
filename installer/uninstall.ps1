#Requires -RunAsAdministrator
<#
.SYNOPSIS
    Uninstalls Native Bangla IME from Windows.
#>

param(
    [switch]$Silent
)

$ErrorActionPreference = "Stop"
$installDir = "$env:ProgramFiles\NativeBanglaIME"
$dllName    = "NativeBanglaIME.dll"

function Write-Step($msg) {
    if (-not $Silent) { Write-Host "  -> $msg" -ForegroundColor Cyan }
}

Write-Host ""
Write-Host "Native Bangla IME Uninstaller" -ForegroundColor Yellow
Write-Host "==============================" -ForegroundColor Yellow
Write-Host ""

$imeDll = Join-Path $installDir $dllName
if (Test-Path $imeDll) {
    Write-Step "Unregistering IME DLL..."
    $result = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/s /u `"$imeDll`"" -Wait -PassThru
    if ($result.ExitCode -ne 0) {
        Write-Warning "regsvr32 /u returned exit code $($result.ExitCode) — continuing anyway"
    }
} else {
    Write-Warning "DLL not found at $imeDll — skipping unregister step"
}

# Remove from PATH
$currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($currentPath -like "*$installDir*") {
    Write-Step "Removing from system PATH"
    $newPath = ($currentPath -split ";" | Where-Object { $_ -ne $installDir }) -join ";"
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
}

# Remove install directory
if (Test-Path $installDir) {
    Write-Step "Removing $installDir"
    Remove-Item $installDir -Recurse -Force
}

Write-Host ""
Write-Host "Uninstallation complete." -ForegroundColor Green
Write-Host "Please log off and log back in for changes to take effect." -ForegroundColor Yellow
Write-Host ""
