# Protocolo IPC — DiffPlayerQC v2 (M1)

Comunicación entre el frontend Svelte y el backend Rust (`src-tauri`).

## Desarrollo local

Desde la **raíz del repositorio** (donde está `src-tauri/`):

```bash
pnpm install --dir frontend
cargo tauri dev
# equivalente: pnpm tauri:dev
```

`cargo tauri` no admite `--manifest-path`; detecta `src-tauri/tauri.conf.json` automáticamente.

## Comandos (`invoke`)

| Comando | Parámetros | Retorno | Descripción |
|---------|------------|---------|-------------|
| `obtener_estado` | — | `SnapshotReproduccion` | Estado actual sin mutar |
| `abrir_video` | `canal`: `"a"` \| `"b"`, `ruta`: string | `SnapshotReproduccion` | Abre archivo por ruta absoluta |
| `abrir_dialogo` | `canal`: `"a"` \| `"b"` | `SnapshotReproduccion` \| null | Diálogo nativo; null si cancela |
| `alternar_play` | — | `SnapshotReproduccion` | Play ↔ Pausa |
| `seek` | `pts`: number (segundos) | `SnapshotReproduccion` | Salto absoluto |
| `step_adelante` | — | `SnapshotReproduccion` | Un fotograma adelante |
| `step_atras` | — | `SnapshotReproduccion` | Un fotograma atrás |

## Eventos (`listen`)

| Evento | Payload | Frecuencia |
|--------|---------|------------|
| `playback-tick` | `SnapshotReproduccion` | ~60 Hz en reproducción (mín. 16 ms); ~10 Hz en pausa |

### `SnapshotReproduccion`

```typescript
{
  pts_actual: number;
  reproduciendo: boolean;
  duracion_a: number;
  duracion_b: number;
  ruta_a: string | null;
  ruta_b: string | null;
  fps: number;
  nivel_audio_a: number;  // 0..1
  nivel_audio_b: number;
}
```

## M1 — fuera de alcance

- Texturas / canvas wgpu (M3)
- Proxy EXR, scopes, command palette (M2+)
