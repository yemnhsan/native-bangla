# generate-assets.ps1 — Creates NSIS banner assets (icon + welcome bitmap)
# Run once to generate installer graphics. Requires .NET (built into Windows).

Add-Type -AssemblyName System.Drawing

$bannerDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# ── Welcome bitmap (164x314 px — NSIS MUI sidebar) ──────────────────────────
$bmp = New-Object System.Drawing.Bitmap(164, 314)
$g   = [System.Drawing.Graphics]::FromImage($bmp)

# Dark gradient background
$topColor = [System.Drawing.Color]::FromArgb(255, 15, 23, 42)     # slate-900
$botColor = [System.Drawing.Color]::FromArgb(255, 30, 64, 175)    # blue-800
$brush    = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
    [System.Drawing.Point]::new(0, 0),
    [System.Drawing.Point]::new(0, 314),
    $topColor, $botColor
)
$g.FillRectangle($brush, 0, 0, 164, 314)
$brush.Dispose()

# Bangla character "ব" as decorative element (large, semi-transparent white)
$fontLarge = New-Object System.Drawing.Font("Vrinda", 80, [System.Drawing.FontStyle]::Bold)
$brush2    = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(40, 255, 255, 255))
$g.DrawString([char]0x09AC, $fontLarge, $brush2, -10, 80)   # ব
$fontLarge.Dispose()
$brush2.Dispose()

# App name text
$fontName  = New-Object System.Drawing.Font("Segoe UI", 9, [System.Drawing.FontStyle]::Bold)
$fontSub   = New-Object System.Drawing.Font("Segoe UI", 7)
$white     = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$gray      = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(180, 255, 255, 255))

$g.DrawString("Native Bangla", $fontName, $white, 8, 240)
$g.DrawString("IME", $fontName, $white, 8, 256)
$g.DrawString("Phonetic Input for Windows", $fontSub, $gray, 8, 276)
$g.DrawString("v0.1.0", $fontSub, $gray, 8, 290)

$fontName.Dispose(); $fontSub.Dispose(); $white.Dispose(); $gray.Dispose()
$g.Dispose()

$welcomePath = Join-Path $bannerDir "welcome.bmp"
$bmp.Save($welcomePath, [System.Drawing.Imaging.ImageFormat]::Bmp)
$bmp.Dispose()
Write-Host "Created: $welcomePath"

# ── Icon (.ico) — 32x32 simple icon ─────────────────────────────────────────
# Create a simple icon with "ব" on dark background
$ico32 = New-Object System.Drawing.Bitmap(32, 32)
$gi    = [System.Drawing.Graphics]::FromImage($ico32)
$gi.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias

# Background circle
$bgBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 30, 64, 175))
$gi.FillEllipse($bgBrush, 0, 0, 31, 31)
$bgBrush.Dispose()

# Letter
$fIco  = New-Object System.Drawing.Font("Vrinda", 18, [System.Drawing.FontStyle]::Bold)
$wBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$gi.DrawString([char]0x09AC, $fIco, $wBrush, 2, 3)
$fIco.Dispose(); $wBrush.Dispose(); $gi.Dispose()

# Save as .ico via memory stream trick
$icoPath = Join-Path $bannerDir "icon.ico"
$ms = New-Object System.IO.MemoryStream

# ICO header
$writer = New-Object System.IO.BinaryWriter($ms)
$writer.Write([uint16]0)      # reserved
$writer.Write([uint16]1)      # type: icon
$writer.Write([uint16]1)      # count: 1 image

# PNG encode the 32x32
$pngMs = New-Object System.IO.MemoryStream
$ico32.Save($pngMs, [System.Drawing.Imaging.ImageFormat]::Png)
$pngBytes = $pngMs.ToArray()
$pngMs.Dispose()
$ico32.Dispose()

# Directory entry
$writer.Write([byte]32)       # width
$writer.Write([byte]32)       # height
$writer.Write([byte]0)        # color count
$writer.Write([byte]0)        # reserved
$writer.Write([uint16]1)      # planes
$writer.Write([uint16]32)     # bit count
$writer.Write([uint32]$pngBytes.Length)
$writer.Write([uint32](6 + 16))  # offset to image data
$writer.Write($pngBytes)
$writer.Flush()

[System.IO.File]::WriteAllBytes($icoPath, $ms.ToArray())
$ms.Dispose()
Write-Host "Created: $icoPath"
Write-Host "Done! Banner assets ready."

