# build.ps1 - Setup dependencies and build DiffPlayerQC on Windows
# Run with: powershell -ExecutionPolicy Bypass -File build.ps1
$ErrorActionPreference = "Stop"

$IsWin = $IsWindows -or ($env:OS -eq "Windows_NT") -or ($PSVersionTable.PSVersion.Major -lt 6)
if (-not $IsWin) {
    Write-Host "This script is for Windows. On macOS/Linux use build.sh or cargo build." -ForegroundColor Yellow
    exit 1
}

Write-Host "=== DiffPlayerQC - Setup and Build (Windows) ===" -ForegroundColor Cyan

# ---------- 1. Rust (rustup + cargo) ----------
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Host "`n[1/4] Rust not found. Installing rustup..." -ForegroundColor Yellow
    $rustupUrl = "https://win.rustup.org/x86_64"
    $rustupExe = "$env:TEMP\rustup-init.exe"
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe -UseBasicParsing
        & $rustupExe -y
        $env:PATH = "$env:USERPROFILE\.cargo\bin;" + $env:PATH
        $cargo = Get-Command cargo -ErrorAction SilentlyContinue
        if (-not $cargo) {
            Write-Host "Rust installed. Please close and reopen PowerShell, then run this script again." -ForegroundColor Green
            exit 0
        }
    } catch {
        Write-Host "Failed to download rustup: $_" -ForegroundColor Red
        Write-Host "Install manually from https://rustup.rs and run this script again." -ForegroundColor Yellow
        exit 1
    }
} else {
    Write-Host "`n[1/4] Rust found: $($cargo.Source)" -ForegroundColor Green
}

# Ensure GNU target for Windows (needed when using MSYS2 gcc)
rustup target add x86_64-pc-windows-gnu 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Adding target x86_64-pc-windows-gnu via rustup..." -ForegroundColor DarkGray
}

# ---------- 2. MSYS2 ----------
$msys64 = "C:\msys64"
$ucrt64 = "$msys64\ucrt64"
$ucrt64Bin = "$ucrt64\bin"

if (-not (Test-Path $ucrt64Bin)) {
    Write-Host "`n[2/4] MSYS2 UCRT64 not found at $msys64" -ForegroundColor Yellow
    # Try winget first
    $winget = Get-Command winget -ErrorAction SilentlyContinue
    if ($winget) {
        Write-Host "Installing MSYS2 via winget (this may take a few minutes)..." -ForegroundColor Cyan
        winget install --id MSYS2.MSYS2 --accept-package-agreements --accept-source-agreements
        if (-not (Test-Path $msys64)) {
            $msys64 = "${env:ProgramFiles}\msys64"
            $ucrt64 = "$msys64\ucrt64"
            $ucrt64Bin = "$ucrt64\bin"
        }
    }
    if (-not (Test-Path $ucrt64Bin)) {
        Write-Host "MSYS2 not found. Please install it manually:" -ForegroundColor Red
        Write-Host "  1. Download from https://www.msys2.org/" -ForegroundColor White
        Write-Host "  2. Run the installer (default: C:\msys64)" -ForegroundColor White
        Write-Host "  3. Open 'MSYS2 UCRT64' from Start Menu and run:" -ForegroundColor White
        Write-Host "     pacman -Syu" -ForegroundColor White
        Write-Host "     pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        Write-Host "  4. Run this script again." -ForegroundColor White
        exit 1
    }
}
Write-Host "[2/4] MSYS2 UCRT64 found: $ucrt64" -ForegroundColor Green

# ---------- 3. MSYS2 packages (FFmpeg, GCC, pkg-config) ----------
Write-Host "`n[3/4] Checking MSYS2 build dependencies..." -ForegroundColor Cyan
$gccExe = "$ucrt64Bin\gcc.exe"
$pkgConfig = "$ucrt64Bin\pkg-config.exe"
$ffmpegPc = "$ucrt64\lib\pkgconfig\libavcodec.pc"

if (-not (Test-Path $gccExe) -or -not (Test-Path $ffmpegPc)) {
    Write-Host "Installing build tools and FFmpeg in MSYS2 UCRT64..." -ForegroundColor Yellow
    $bash = "$msys64\usr\bin\bash.exe"
    if (-not (Test-Path $bash)) {
        Write-Host "bash.exe not found. Open 'MSYS2 UCRT64' and run:" -ForegroundColor Red
        Write-Host "  pacman -Syu" -ForegroundColor White
        Write-Host "  pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        exit 1
    }
    & $bash -lc "pacman -Syu --noconfirm"
    & $bash -lc "pacman -S --noconfirm mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf"
    if ($LASTEXITCODE -ne 0) {
        Write-Host "pacman install failed. Run manually in MSYS2 UCRT64:" -ForegroundColor Red
        Write-Host "  pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        exit 1
    }
    Write-Host "MSYS2 packages installed." -ForegroundColor Green
} else {
    Write-Host "[3/4] Build dependencies (gcc, ffmpeg, pkg-config) OK." -ForegroundColor Green
}

# ---------- 4. Build ----------
Write-Host "`n[4/4] Building DiffPlayerQC (Release)..." -ForegroundColor Cyan

# Unset FFMPEG_DIR so ffmpeg-sys-next uses pkg-config
$env:FFMPEG_DIR = $null
$env:PATH = "$ucrt64Bin;" + $env:PATH
$env:PKG_CONFIG_PATH = "$ucrt64\lib\pkgconfig"
$env:PKG_CONFIG_ALLOW_CROSS = "1"
$env:LIBCLANG_PATH = $ucrt64Bin
$env:CC = "gcc"

Push-Location $PSScriptRoot
try {
    cargo build --release --target x86_64-pc-windows-gnu
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed." -ForegroundColor Red
        exit $LASTEXITCODE
    }

    $VERSION = (Select-String -Path "Cargo.toml" -Pattern '^version = "(.*)"').Matches.Groups[1].Value
    Write-Host "`nBuild successful (version $VERSION)." -ForegroundColor Green

    $RELEASE_DIR = "target\x86_64-pc-windows-gnu\release"
    $DIST_DIR = "dist\Windows_v$VERSION"
    New-Item -ItemType Directory -Force -Path $DIST_DIR | Out-Null

    $appName = "diffplayerqc.exe"
    $runningProcesses = Get-Process -Name ($appName -replace "\.exe$", "") -ErrorAction SilentlyContinue
    if ($runningProcesses) {
        Write-Host "Closing running instances of $appName..." -ForegroundColor Cyan
        $runningProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 500
    }

    if (Test-Path $DIST_DIR) { Remove-Item -Recurse -Force $DIST_DIR -ErrorAction SilentlyContinue }
    New-Item -ItemType Directory -Force -Path $DIST_DIR | Out-Null

    Copy-Item "$RELEASE_DIR\diffplayerqc.exe" -Destination "$DIST_DIR\diffplayerqc-v$VERSION.exe"

    # Copy required DLLs from MSYS2
    $LDD_EXE = "$msys64\usr\bin\ldd.exe"
    if (Test-Path $LDD_EXE) {
        $lddOutput = & $LDD_EXE "$DIST_DIR\diffplayerqc-v$VERSION.exe" 2>$null
        $count = 0
        foreach ($line in $lddOutput) {
            if ($line -match "=>\s+(/ucrt64/bin/.*?\.dll)") {
                $winPath = ($matches[1] -replace "^/ucrt64/", "$ucrt64\") -replace "/", "\"
                if (Test-Path $winPath) {
                    Copy-Item $winPath -Destination "$DIST_DIR\" -ErrorAction SilentlyContinue
                    $count++
                }
            }
        }
        Write-Host "Copied $count DLLs to $DIST_DIR" -ForegroundColor Green
    }

    Write-Host "`nOutput: $DIST_DIR\diffplayerqc-v$VERSION.exe" -ForegroundColor Cyan
    Write-Host "Dist folder ready: $DIST_DIR" -ForegroundColor White
} finally {
    Pop-Location
}
