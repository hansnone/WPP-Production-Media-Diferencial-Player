#!/usr/bin/env bash
# build.sh - Autonomously-bundled MacOS / Linux Build Script for DiffPlayerQC

set -e

OS="$(uname -s)"
echo -e "\033[1;36m========================================================\033[0m"
echo -e "\033[1;36m Construyendo WPP DiffPlayerQC Portable para $OS        \033[0m"
echo -e "\033[1;36m========================================================\033[0m"

echo -e "\n\033[1;33m[1/3] Compilando codigo Rust (Modo Release)...\033[0m"

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
    
    APP_NAME="WPP DiffPlayerQC.app"
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
    <string>1.2.5</string>
    <key>CFBundleShortVersionString</key>
    <string>1.2.5</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
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
    
    echo -e "\n\033[1;33m[Opcional] Generando Instalador .pkg ...\033[0m"
    pkgbuild --component "$DIST_DIR/$APP_NAME" \
             --install-location /Applications \
             "$DIST_DIR/WPP_DiffPlayerQC_Installer.pkg" || true
             
    echo -e "\n\033[1;32m========================================================\033[0m"
    echo -e "\033[1;32m¡Empaquetado macOS sin dependencias externas completado!\033[0m"
    echo -e "- Aplicacion Lista para entregar: \033[1;37m$DIST_DIR/$APP_NAME\033[0m"
    echo -e "\033[1;32m========================================================\033[0m"

elif [ "$OS" = "Linux" ]; then
    echo -e "\n\033[1;33m[2/3] Plataforma Linux. Preparando ejecutable...\033[0m"
    DIST_DIR="dist/Linux"
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR/libs"
    
    cp target/release/diffplayerqc "$DIST_DIR/wpp-diffplayerqc"
    chmod +x "$DIST_DIR/wpp-diffplayerqc"
    
    echo -e "\n\033[1;33m[3/3] Empaquetando dependencias SO (.so)...\033[0m"
    # LDD en unix rastrea librerias. Copiamos las que no son del kernel.
    ldd "$DIST_DIR/wpp-diffplayerqc" | grep -E "libav|libsw" | awk '{print $3}' | while read -r lib; do
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
