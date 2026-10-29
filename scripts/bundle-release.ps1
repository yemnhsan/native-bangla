#!/usr/bin/env powershell
# bundle-release.ps1 — Bundles the Native Bangla IME release package locally.
# Run after: cargo build --workspace --release (and NativeBanglaIME.dll is built)

param(
    [string]$Version = "0.1.0",
    [string]$LlvmBin = ""
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $MyInvocation.MyCommand.Path | Split-Path -Parent

# Auto-detect llvm-mingw bin dir
if (-not $LlvmBin) {
    $winget = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages"
    if (Test-Path $winget) {
        $llvmPkg = Get-ChildItem $winget -Filter "MartinStorsjo.LLVM-MinGW*" -Directory -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($llvmPkg) {
            $inner = Get-ChildItem $llvmPkg.FullName -Filter "llvm-mingw-*-x86_64" -Directory -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($inner) { $LlvmBin = "$($inner.FullName)\x86_64-w64-mingw32\bin" }
        }
    }
}

$releaseDir = "$root\dist\release"
$zipPath    = "$root\dist\NativeBangla-v${Version}-windows-x64.zip"

Write-Host "Building release package v$Version..." -ForegroundColor Cyan

# Clean and create output dir
if (Test-Path $releaseDir) { Remove-Item $releaseDir -Recurse -Force }
New-Item -ItemType Directory -Force $releaseDir | Out-Null
New-Item -ItemType Directory -Force "$root\dist"  | Out-Null

# Core DLLs
foreach ($dll in @("NativeBanglaIME.dll", "ime_bridge.dll")) {
    $src = "$root\target\release\$dll"
    if (-not (Test-Path $src)) { Write-Error "Missing: $src"; exit 1 }
    Copy-Item $src $releaseDir
    Write-Host "  Copied $dll"
}

# libunwind.dll from llvm-mingw
if ($LlvmBin -and (Test-Path "$LlvmBin\libunwind.dll")) {
    Copy-Item "$LlvmBin\libunwind.dll" $releaseDir
    Write-Host "  Copied libunwind.dll from $LlvmBin"
} else {
    Write-Warning "libunwind.dll not found (LlvmBin=$LlvmBin) -- bundling skipped"
}

# Seed dictionary
$dictSrc = "$root\resources\dictionaries\bn-core-seed.json"
if (Test-Path $dictSrc) {
    New-Item -ItemType Directory -Force "$releaseDir\resources\dictionaries" | Out-Null
    Copy-Item $dictSrc "$releaseDir\resources\dictionaries\"
    Write-Host "  Copied bn-core-seed.json"
}

# Installer scripts
Copy-Item "$root\installer\install.ps1"   $releaseDir
Copy-Item "$root\installer\uninstall.ps1" $releaseDir

# README (ASCII-safe to avoid PowerShell heredoc encoding issues)
$readme = @(
    "Native Bangla IME v${Version}",
    "================================",
    "",
    "INSTALLATION (requires Administrator)",
    "--------------------------------------",
    "1. Right-click install.ps1 and choose 'Run with PowerShell' (as Administrator)",
    "   OR open an elevated PowerShell and run:",
    "      Set-ExecutionPolicy Bypass -Scope Process -Force",
    "      .\install.ps1",
    "",
    "UNINSTALL",
    "---------",
    "   .\uninstall.ps1   (as Administrator)",
    "",
    "REQUIREMENTS",
    "------------",
    "- Windows 10 or later (x64)",
    "- Bengali (Bangladesh) language pack installed in Windows Settings",
    "",
    "AFTER INSTALL",
    "-------------",
    "Go to: Settings > Time & Language > Language > Add a language",
    "Add Bengali (Bangladesh) and select 'Native Bangla IME' as input method."
)
$readme | Set-Content "$releaseDir\README.txt" -Encoding UTF8

# Zip it up
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
Compress-Archive -Path "$releaseDir\*" -DestinationPath $zipPath
Write-Host ""
Write-Host "Zip package ready:" -ForegroundColor Green
Write-Host "  $zipPath"
Write-Host "  Size: $([math]::Round((Get-Item $zipPath).Length / 1MB, 2)) MB"

# Build Inno Setup installer (.exe) if ISCC is installed
$isccCmd = Get-Command iscc -ErrorAction SilentlyContinue
$iscc = if ($isccCmd) { $isccCmd.Source } else { $null }
if (-not $iscc) {
    $candidates = @(
        "$env:LOCALAPPDATA\Programs\Inno Setup 7\ISCC.exe",
        "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
        "C:\Program Files\Inno Setup 6\ISCC.exe"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $iscc = $c; break }
    }
}

if ($iscc) {
    Write-Host ""
    Write-Host "Compiling Windows Installer (.exe) with Inno Setup..." -ForegroundColor Cyan
    $issFile = "$root\installer\inno\NativeBangla.iss"
    & $iscc "/dMyAppVersion=$Version" $issFile
    $exePath = "$root\dist\NativeBangla-v${Version}-Setup.exe"
    if (Test-Path $exePath) {
        Write-Host "Windows Setup Installer ready:" -ForegroundColor Green
        Write-Host "  $exePath"
        Write-Host "  Size: $([math]::Round((Get-Item $exePath).Length / 1MB, 2)) MB"
    }
} else {
    Write-Warning "Inno Setup compiler (ISCC.exe) not found. Skipped building .exe installer."
}
