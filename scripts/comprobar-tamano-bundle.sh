#!/usr/bin/env bash
# Comprueba el tamaño del .dmg generado (objetivo M7: < 25 MB en macOS arm64).
set -euo pipefail

LIMITE_MB="${LIMITE_MB:-25}"
BASE="${1:-src-tauri/target/release-small/bundle}"

DMG=$(find "$BASE" -name '*.dmg' 2>/dev/null | head -1)
if [[ -z "$DMG" ]]; then
  echo "No se encontró .dmg en $BASE" >&2
  exit 1
fi

MB=$(du -m "$DMG" | cut -f1)
echo "DMG: $DMG"
echo "Tamaño: ${MB} MB (límite ${LIMITE_MB} MB)"

if [[ "$MB" -gt "$LIMITE_MB" ]]; then
  echo "ERROR: supera el límite." >&2
  exit 1
fi

echo "OK."
