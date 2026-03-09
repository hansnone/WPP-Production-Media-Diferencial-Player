# build.ps1 - Cross-platform (Windows) build script for DiffPlayerQC
$ErrorActionPreference = "Stop"

# Detect if we are on Windows
$IsWin = $IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6)

if ($IsWin) {
    Write-Host "Detected OS: Windows"
    
    $msys2_path = "C:\msys64\ucrt64"
    if (-not (Test-Path $msys2_path)) {
        Write-Host "MSYS2 UCRT64 environment not found at $msys2_path." -ForegroundColor Red
        Write-Host "Please ensure MSYS2 is installed along with dependencies:"
        Write-Host "pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-clang mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf"
        exit 1
    }

    Write-Host "Configuring MSYS2 UCRT64 environment variables..." -ForegroundColor DarkGray

    # CRITICAL: We MUST remove FFMPEG_DIR so ffmpeg-sys-next uses pkg-config instead of its fragile path fallback.
    $env:FFMPEG_DIR = $null

    # Prepend MSYS2 bin to PATH so gcc and pkg-config commands resolve to the MSYS2 ones
    $env:PATH = "$msys2_path\bin;" + $env:PATH

    # Set paths for pkg-config and bindgen
    $env:PKG_CONFIG_PATH = "$msys2_path\lib\pkgconfig"
    $env:PKG_CONFIG_ALLOW_CROSS = "1"
    $env:LIBCLANG_PATH = "$msys2_path\bin"
    $env:CC = "gcc"

    Write-Host "Compiling DiffPlayerQC for Windows (GNU target - Release Mode)..." -ForegroundColor Cyan
    cargo build --release --target x86_64-pc-windows-gnu

    if ($LASTEXITCODE -eq 0) {
        # Extraer versión de Cargo.toml
        $VERSION = (Select-String -Path "Cargo.toml" -Pattern '^version = "(.*)"').Matches.Groups[1].Value
        Write-Host "Version detectada: $VERSION" -ForegroundColor Green

        Write-Host "Build successful!" -ForegroundColor Green
        
        $RELEASE_DIR = "target\x86_64-pc-windows-gnu\release"
        $DIST_DIR = "dist\Windows_v$VERSION"
        
        Write-Host "`nPreparando carpeta de distribucion portable en $DIST_DIR..." -ForegroundColor Yellow
        
        # Kill running instances if any
        $appName = "diffplayerqc.exe"
        $runningProcesses = Get-Process -Name ($appName -replace "\.exe$", "") -ErrorAction SilentlyContinue
        if ($runningProcesses) {
            Write-Host "Cerrando instancias activas de $appName..." -ForegroundColor Cyan
            $runningProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
            Start-Sleep -Milliseconds 500
        }

        if (Test-Path $DIST_DIR) {
            Write-Host "Limpiando directorio de distribucion..." -ForegroundColor DarkGray
            try {
                Remove-Item -Recurse -Force $DIST_DIR -ErrorAction Stop
            } catch {
                # Windows trick: Renaming a locked folder often works where Move or Delete fails
                # because Explorer handles are often tied to the path/name.
                $tempDir = $DIST_DIR + "_" + (Get-Date -Format "HHmmss")
                Write-Host "Aviso: Carpeta bloqueada. Intentando maniobra de escape (renombrado a $tempDir)..." -ForegroundColor Gray
                try {
                    Rename-Item -Path $DIST_DIR -NewName (Split-Path $tempDir -Leaf) -ErrorAction Stop
                    # Once renamed, we can try to delete it in the background or just leave it.
                    # We'll try one last time to delete the renamed one.
                    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
                } catch {
                    Write-Host "ERROR: $DIST_DIR sigue bloqueado por otro proceso de forma persistente." -ForegroundColor Red
                    Write-Host "Cierra exploradores de archivos o editores abiertos en esa ruta y reintenta." -ForegroundColor Yellow
                    throw "Acceso denegado a $DIST_DIR"
                }
            }
        }
        New-Item -ItemType Directory -Force -Path $DIST_DIR | Out-Null
        
        Write-Host "Copiando ejecutable hiper-optimizado..." -ForegroundColor Green
        Copy-Item "$RELEASE_DIR\diffplayerqc.exe" -Destination "$DIST_DIR\diffplayerqc-v$VERSION.exe"

        Write-Host "Recopilando dlls dinamicas requeridas desde MSYS2..." -ForegroundColor Yellow
        $LDD_EXE = "C:\msys64\usr\bin\ldd.exe"

        if (-Not (Test-Path $LDD_EXE)) {
            Write-Host "Error: No se encontro ldd.exe en MSYS2. Saltando copiado de dlls." -ForegroundColor Red
        } else {
            $lddOutput = & $LDD_EXE "$DIST_DIR\diffplayerqc-v$VERSION.exe" 2> $null
            $contadorDll = 0
            
            foreach ($line in $lddOutput) {
                if ($line -match "=>\s+(/ucrt64/bin/.*?\.dll)") {
                    $winPath = ($matches[1] -replace "^/ucrt64/", "C:\msys64\ucrt64\") -replace "/", "\"
                    
                    if (Test-Path $winPath) {
                        Copy-Item $winPath -Destination "$DIST_DIR\" -ErrorAction SilentlyContinue
                        $contadorDll++
                    }
                }
            }
            Write-Host "Extraccion completada: $contadorDll DLLs esenciales nativas de FFmpeg copiadas al instalador." -ForegroundColor Green
            Write-Host "`n==========================================================================" -ForegroundColor Cyan
            Write-Host "¡La app ya es MUY PORTABLE para cualquier cliente de Windows!" -ForegroundColor White 
            Write-Host "Simplemente comprime la carpeta `$(Resolve-Path $DIST_DIR).Path` y envíasela a cualquiera." -ForegroundColor White
            Write-Host "==========================================================================" -ForegroundColor Cyan
        }

    } else {
        Write-Host "Build failed with exit code $LASTEXITCODE" -ForegroundColor Red
        exit $LASTEXITCODE
    }
} else {
    Write-Host "You are running PowerShell on a non-Windows OS." -ForegroundColor Yellow
    Write-Host "Please use ./build.sh instead."
    exit 1
}
