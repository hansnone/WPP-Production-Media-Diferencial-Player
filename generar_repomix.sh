#!/bin/bash
# Este script genera un archivo markdown con todo el código fuente del proyecto
# ignorando automáticamente los archivos en .gitignore (como la carpeta target/)

echo "Generando snapshot del código con repomix..."

# --output: nombre del archivo de salida
# Puedes cambiar la extensión a .txt si prefieres texto plano en lugar de markdown
npx repomix --output repomix_codigo.txt

echo "¡Completado! El código se ha guardado en 'repomix_codigo.md'"
