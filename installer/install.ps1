#Requires -RunAsAdministrator
<#
.SYNOPSIS
    Installs Native Bangla IME on Windows.
.DESCRIPTION
    Copies DLLs to System32 and registers the IME with Windows Text Services Framework.
    Must be run as Administrator.
#>

param(
    [switch]$Silent
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$dllName   = "NativeBanglaIME.dll"
$bridgeName = "ime_bridge.dll"
$unwindName = "libunwind.dll"

function Write-Step($msg) {
    if (-not $Silent) { Write-Host "  -> $msg" -ForegroundColor Cyan }
}

Write-Host ""
Write-Host "Native Bangla IME Installer" -ForegroundColor Green
Write-Host "===========================" -ForegroundColor Green
Write-Host ""

# Validate source files
foreach ($f in @($dllName, $bridgeName, $unwindName)) {
    $src = Join-Path $scriptDir $f
    if (-not (Test-Path $src)) {
        Write-Error "Missing file: $src`nPlease ensure all DLLs are in the same folder as install.ps1"
        exit 1
    }
}

# Copy DLLs to a dedicated folder in Program Files
$installDir = "$env:ProgramFiles\NativeBanglaIME"
Write-Step "Creating install directory: $installDir"
New-Item -ItemType Directory -Force $installDir | Out-Null

foreach ($f in @($dllName, $bridgeName, $unwindName)) {
    Write-Step "Copying $f"
    Copy-Item (Join-Path $scriptDir $f) $installDir -Force
}

# Copy seed dictionary
$dictSrc = Join-Path $scriptDir "resources\dictionaries\bn-core-seed.json"
if (Test-Path $dictSrc) {
    Write-Step "Copying seed dictionary"
    New-Item -ItemType Directory -Force "$installDir\resources\dictionaries" | Out-Null
    Copy-Item $dictSrc "$installDir\resources\dictionaries\" -Force
}

# Add install dir to system PATH (so DLLs are found at runtime)
$currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($currentPath -notlike "*$installDir*") {
    Write-Step "Adding to system PATH"
    [System.Environment]::SetEnvironmentVariable("PATH", "$currentPath;$installDir", "Machine")
}

# Register the IME DLL with regsvr32
$imeDll = Join-Path $installDir $dllName
Write-Step "Registering IME DLL (regsvr32)..."
$result = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/s `"$imeDll`"" -Wait -PassThru
if ($result.ExitCode -ne 0) {
    Write-Error "regsvr32 failed with exit code $($result.ExitCode)"
    exit $result.ExitCode
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "Please log off and log back in (or restart) to activate Native Bangla IME." -ForegroundColor Yellow
Write-Host "Then go to: Settings > Time & Language > Language > Preferred languages"
Write-Host "Add Bengali (Bangladesh) and select 'Native Bangla IME' as the input method."
Write-Host ""

