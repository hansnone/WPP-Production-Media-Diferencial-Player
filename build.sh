#!/usr/bin/env bash
# build.sh - Autonomously-bundled MacOS / Linux Build Script for DiffPlayerQC

set -e

OS="$(uname -s)"
echo -e "\033[1;36m========================================================\033[0m"
echo -e "\033[1;36m Construyendo WPP DiffPlayerQC Portable para $OS        \033[0m"
echo -e "\033[1;36m========================================================\033[0m"

echo -e "\n\033[1;33m[1/3] Compilando codigo Rust (Modo Release)...\033[0m"

# Extraer versión de Cargo.toml
VERSION=$(grep "^version =" Cargo.toml | head -n 1 | cut -d '"' -f 2)
echo -e "\033[1;32mVersion detectada: $VERSION\033[0m"

# Configuración para macOS con Homebrew y FFmpeg@7
if [ "$OS" = "Darwin" ]; then
    if brew --prefix ffmpeg@7 >/dev/null 2>&1; then
        export FFMPEG_DIR=$(brew --prefix ffmpeg@7)
        export PKG_CONFIG_PATH="/opt/homebrew/opt/ffmpeg@7/lib/pkgconfig"
        export BINDGEN_EXTRA_CLANG_ARGS="-I${FFMPEG_DIR}/include"
        echo -e "\033[1;32mEntorno configurado para FFmpeg@7 detectado en $FFMPEG_DIR\033[0m"
    fi
fi

cargo build --release

if [ "$OS" = "Darwin" ]; then
    echo -e "\n\033[1;33m[2/3] Plataforma macOS. Empacando como .app standalone...\033[0m"
    
    APP_NAME="WPP DiffPlayerQC v$VERSION.app"
    DIST_DIR="dist/macOS"
    CONTENTS="$DIST_DIR/$APP_NAME/Contents"
    BINS="$CONTENTS/MacOS"
    RES="$CONTENTS/Resources"
    LIBS="$CONTENTS/Frameworks"
    
    rm -rf "$DIST_DIR"
    mkdir -p "$BINS" "$RES" "$LIBS"
    
    cat <<EOF > "$CONTENTS/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>diffplayerqc</string>
    <key>CFBundleIdentifier</key>
    <string>com.wpp.diffplayerqc</string>
    <key>CFBundleName</key>
    <string>WPP DiffPlayerQC</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF
    
    # 1. Copiamos el ejecutable y los iconos dentro del contenedor final
    cp target/release/diffplayerqc "$BINS/"
    if [ -f "assets/AppIcon.icns" ]; then
        cp "assets/AppIcon.icns" "$RES/"
    fi
    
    echo -e "\n\033[1;33m[3/3] Incorporando dependencias (.dylib) internamente a la App...\033[0m"
    # Este paso rastrea (otool) todas las dylib del sistema (ffmpeg) de las que tira el binario,
    # y las mete dentro de Frameworks alterando las rutas absolutas para que sea portable.
    if ! command -v dylibbundler &> /dev/null; then
        echo -e "\033[1;35mAviso: dylibbundler no instalado.\033[0m"
        echo -e "Como solucion rapida incrustando con Otool manual..."
        
        # Copiar librerias de FFmpeg al interior de la aplicacion de forma manual si es posible
        for lib in $(otool -L "$BINS/diffplayerqc" | grep -E "libav|libsw|libpostproc" | awk '{print $1}'); do
            if [ -f "$lib" ]; then
                cp "$lib" "$LIBS/"
                install_name_tool -change "$lib" "@executable_path/../Frameworks/$(basename "$lib")" "$BINS/diffplayerqc"
            fi
        done
        echo "Librerías principales empaquetadas."
    else
        dylibbundler -b -x "$BINS/diffplayerqc" -d "$LIBS/" -p "@executable_path/../Frameworks/"
    fi
    
    # [HOTFIX] Homebrew's sdl2-compat requires SDL3 at runtime via dlopen.
    # dylibbundler misses it porque no analiza llamadas dinámicas (dlopen).
    if [ -f "/opt/homebrew/lib/libSDL3.dylib" ]; then
        cp -L "/opt/homebrew/lib/libSDL3.dylib" "$LIBS/libSDL3.dylib"
        chmod +w "$LIBS/libSDL3.dylib"
        install_name_tool -id "@executable_path/../Frameworks/libSDL3.dylib" "$LIBS/libSDL3.dylib"
        codesign --force --sign - "$LIBS/libSDL3.dylib"
    fi
    
    echo -e "\n\033[1;33m[4/3] Preparando instalador con integración Youlean...\033[0m"
    SCRIPTS_DIR="$DIST_DIR/scripts"
    rm -rf "$SCRIPTS_DIR"
    mkdir -p "$SCRIPTS_DIR/Settings"
    
    cp "assets/Youlean-Loudness-Meter-2-V2.5.14-macOS-1.dmg" "$SCRIPTS_DIR/Youlean.dmg"
    cp -R "assets/youlean_settings/"* "$SCRIPTS_DIR/Settings/"
    
    cat <<'EOF' > "$SCRIPTS_DIR/postinstall"
#!/bin/bash
DIR=$(dirname "$0")

hdiutil attach "$DIR/Youlean.dmg" -nobrowse -mountpoint /tmp/youlean_mount
installer -pkg "/tmp/youlean_mount/Youlean Loudness Meter 2 - Installer.pkg" -target "$3"
hdiutil detach /tmp/youlean_mount -force

CONSOLE_USER=$(stat -f "%Su" /dev/console)
if [ "$CONSOLE_USER" != "root" ]; then
    USER_HOME=$(dscl . -read /Users/$CONSOLE_USER NFSHomeDirectory | awk '{print $2}')
    YOULEAN_DIR="$USER_HOME/Library/Application Support/Youlean/Youlean Loudness Meter 2"
    mkdir -p "$YOULEAN_DIR"
    cp -R "$DIR/Settings/"* "$YOULEAN_DIR/"
    chown -R $CONSOLE_USER "$YOULEAN_DIR"
fi
exit 0
EOF
    chmod +x "$SCRIPTS_DIR/postinstall"
    
    pkgbuild --component "$DIST_DIR/$APP_NAME" \
             --install-location /Applications \
             --scripts "$SCRIPTS_DIR" \
             "$DIST_DIR/WPP_DiffPlayerQC_Installer_v$VERSION.pkg" || true
             
    echo -e "\n\033[1;32m========================================================\033[0m"
    echo -e "\033[1;32m¡Empaquetado macOS sin dependencias externas completado!\033[0m"
    echo -e "- Aplicacion Lista para entregar: \033[1;37m$DIST_DIR/$APP_NAME\033[0m"
    echo -e "\033[1;32m========================================================\033[0m"

elif [ "$OS" = "Linux" ]; then
    echo -e "\n\033[1;33m[2/3] Plataforma Linux. Preparando ejecutable...\033[0m"
    DIST_DIR="dist/Linux"
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR/libs"
    
    cp target/release/diffplayerqc "$DIST_DIR/wpp-diffplayerqc-v$VERSION"
    chmod +x "$DIST_DIR/wpp-diffplayerqc-v$VERSION"
    
    echo -e "\n\033[1;33m[3/3] Empaquetando dependencias SO (.so)...\033[0m"
    # LDD en unix rastrea librerias. Copiamos las que no son del kernel.
    ldd "$DIST_DIR/wpp-diffplayerqc-v$VERSION" | grep -E "libav|libsw" | awk '{print $3}' | while read -r lib; do
        if [ -f "$lib" ]; then
            cp "$lib" "$DIST_DIR/libs/"
        fi
    done
    
    echo -e "\n\033[1;32m========================================================\033[0m"
    echo -e "\033[1;32m¡Linux Build Completado!\033[0m"
    echo -e "Binaros y Librerias en: \033[1;37m$DIST_DIR/\033[0m"
    echo -e "Asegurate de lanzar el programa usando LD_LIBRARY_PATH=./libs ./wpp-diffplayerqc"
    echo -e "\033[1;32m========================================================\033[0m"
fi
