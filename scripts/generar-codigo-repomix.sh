#!/usr/bin/env bash
# Descripción: genera un único archivo con todo el código del repo usando Repomix.
# Uso: ./scripts/generar-codigo-repomix.sh [opciones]
#   -o, --salida <ruta>     Archivo de salida (defecto: dist/repomix-codigo.md)
#   -s, --estilo <formato>   markdown | xml | json | plain (defecto: markdown)
#   -c, --comprimir         Solo estructura esencial (Tree-sitter, menos tokens)
#   -d, --con-diffs         Incluye git diff (cambios sin commitear)
#   --sin-resumen           Omite resumen y árbol de directorios
#   -h, --ayuda             Muestra esta ayuda
# Dependencias: Node.js y npx (descarga repomix@latest si no está instalado)
set -euo pipefail

RAIZ="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$RAIZ"

ARCHIVO_SALIDA="${RAIZ}/dist/repomix-codigo.md"
ESTILO="markdown"
COMPRIMIR=false
CON_DIFFS=false
SIN_RESUMEN=false

mostrar_ayuda() {
  sed -n '2,10p' "$0" | sed 's/^# \{0,1\}//'
  echo ""
  echo "Ejemplos:"
  echo "  ./scripts/generar-codigo-repomix.sh"
  echo "  ./scripts/generar-codigo-repomix.sh -o /tmp/diffplayerqc.xml -s xml"
  echo "  ./scripts/generar-codigo-repomix.sh --comprimir -o dist/repomix-resumen.md"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -o | --salida)
      ARCHIVO_SALIDA="${2:?Falta ruta tras $1}"
      shift 2
      ;;
    -s | --estilo)
      ESTILO="${2:?Falta formato tras $1}"
      shift 2
      ;;
    -c | --comprimir)
      COMPRIMIR=true
      shift
      ;;
    -d | --con-diffs)
      CON_DIFFS=true
      shift
      ;;
    --sin-resumen)
      SIN_RESUMEN=true
      shift
      ;;
    -h | --ayuda)
      mostrar_ayuda
      exit 0
      ;;
    *)
      echo "Error: opción desconocida: $1" >&2
      mostrar_ayuda >&2
      exit 2
      ;;
  esac
done

# Rutas absolutas para salida relativa al cwd del usuario
case "$ARCHIVO_SALIDA" in
  /*) ;;
  *) ARCHIVO_SALIDA="${RAIZ}/${ARCHIVO_SALIDA}" ;;
esac

mkdir -p "$(dirname "$ARCHIVO_SALIDA")"

if ! command -v npx >/dev/null 2>&1; then
  echo "Error: se necesita npx (instala Node.js)." >&2
  exit 1
fi

# Patrones extra además de .gitignore (binarios, salidas previas, vídeos de prueba).
PATRONES_IGNORAR=(
  "**/repomix-output*"
  "**/repomix-codigo*"
  "dist/repomix-*"
  "videos-muestra/**"
  "Binder1.pdf"
  "frontend/test-results/**"
  "frontend/playwright-report/**"
  "**/*.dmg"
  "**/*.app"
)

ARGS=(
  --output "$ARCHIVO_SALIDA"
  --style "$ESTILO"
  --parsable-style
  --output-show-line-numbers
  --truncate-base64
)

for patron in "${PATRONES_IGNORAR[@]}"; do
  ARGS+=(--ignore "$patron")
done

if [[ "$COMPRIMIR" == true ]]; then
  ARGS+=(--compress)
fi

if [[ "$CON_DIFFS" == true ]]; then
  ARGS+=(--include-diffs)
fi

if [[ "$SIN_RESUMEN" == true ]]; then
  ARGS+=(--no-file-summary --no-directory-structure)
fi

# Incluir código fuente principal (Repomix también respeta .gitignore).
ARGS+=(
  --include "src/**,src-tauri/**,frontend/src/**,crates/**,docs/**,scripts/**,shaders/**,*.toml,*.json,*.md,*.yml,*.wgsl"
)

echo "Repomix: empaquetando ${RAIZ}" >&2
echo "  Salida: ${ARCHIVO_SALIDA}" >&2
echo "  Estilo: ${ESTILO}" >&2

npx --yes repomix@latest "${ARGS[@]}"

if [[ ! -f "$ARCHIVO_SALIDA" ]]; then
  echo "Error: no se creó el archivo de salida." >&2
  exit 1
fi

TAMANO="$(du -h "$ARCHIVO_SALIDA" | cut -f1)"
LINEAS="$(wc -l < "$ARCHIVO_SALIDA" | tr -d ' ')"
echo "Listo: ${ARCHIVO_SALIDA} (${TAMANO}, ${LINEAS} líneas)" >&2
