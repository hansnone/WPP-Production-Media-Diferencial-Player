# Release v2 (M7) — DiffPlayerQC Tauri

Guía para mantenedores: builds locales, CI y GitHub Releases.

## Versiones

| Componente | Fuente de verdad |
|------------|------------------|
| App bundle | `src-tauri/tauri.conf.json` → `version` |
| Crate Tauri | `src-tauri/Cargo.toml` |
| Monorepo npm | `package.json` (raíz) |

Al etiquetar, usa **el mismo número** en todos: `v2.0.0` → tag Git `v2.0.0`.

## Build local (macOS)

Requisitos: Rust estable, Node 22+, pnpm 9+, FFmpeg (`brew install ffmpeg`).

```bash
pnpm install --dir frontend
pnpm build
# Perfil optimizado para tamaño (< 25 MB objetivo en .dmg)
cargo tauri build --profile release-small
```

Artefactos en `src-tauri/target/release-small/bundle/` (o `release/` si usas perfil por defecto).

### Por qué el bundle no incluye egui

El binario **v1** (eframe) vive en el crate `diffplayerqc` con feature `egui-app`.  
Tauri declara:

```toml
diffplayerqc = { path = "..", default-features = false }
```

Así el `.app` no enlaza egui/eframe (~decenas de MB menos).

## Firmado macOS (opcional)

Para distribución fuera del Mac de desarrollo:

1. Certificado **Developer ID Application** en Keychain.
2. En GitHub → Settings → Secrets, añade:

| Secret | Contenido |
|--------|-----------|
| `APPLE_CERTIFICATE` | `.p12` en base64 |
| `APPLE_CERTIFICATE_PASSWORD` | Contraseña del p12 |
| `APPLE_SIGNING_IDENTITY` | Nombre del certificado (ej. `Developer ID Application: …`) |
| `KEYCHAIN_PASSWORD` | Contraseña temporal del llavero CI |

3. El workflow [`.github/workflows/release.yml`](../.github/workflows/release.yml) firma si existen los secrets.

Sin secrets, el build es **válido pero no firmado** (Gatekeeper puede bloquear en otros Macs).

Notarización (Apple): paso adicional no automatizado en M7; ver [documentación Tauri](https://v2.tauri.app/distribute/sign/macos/).

## Publicar en GitHub Releases

```bash
git tag v2.0.0
git push origin v2.0.0
```

El workflow `Release`:

1. Valida el formato del tag.
2. Compila en macOS (arm64 + x64), Linux y Windows.
3. Crea un **release en borrador** con los instaladores adjuntos.
4. En arm64 macOS, avisa si el `.dmg` supera 25 MB.

Revisa el draft en GitHub, edita notas si hace falta y publica.

## Reducir tamaño del bundle

- Usar `--profile release-small` (ya en CI).
- No activar `default-features` de `diffplayerqc` en Tauri (sin `egui-app`).
- `strip = true` y `lto = true` en `[profile.release]` del workspace (ya activo).
- Evitar depender de crates pesados solo usados en v1.

## v1 (egui) — build aparte

```bash
cargo build --release --features egui-app -p diffplayerqc
```

No se empaqueta con el workflow Tauri v2.
