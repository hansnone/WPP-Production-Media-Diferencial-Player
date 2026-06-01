# M10 — Eventos QC y notas

## Modelo

- `EventoQc`: id, tipo (`manual` | `video` | `audio`), `pts_secs`, título, descripción opcional, notas.
- `NotaQc`: texto anclado a un PTS (puede coincidir con el del evento).
- `RegistroEventosQc`: colección por **clave de proyecto** (hash de rutas A+B).

Código: `crates/core/src/eventos_qc.rs`.

## Persistencia

- Tauri: `{app_data}/eventos/eventos-{clave}.json`
- Navegador (dev): `localStorage` con prefijo `diffplayerqc-v2-eventos-`

Al abrir o cambiar A/B se llama `actualizar_proyecto_eventos` y se carga el JSON correspondiente.

## IPC

| Comando | Uso |
|---------|-----|
| `actualizar_proyecto_eventos` | Cambiar par A/B activo |
| `listar_eventos` | Lista con filtro opcional por tipo |
| `crear_evento` | Nuevo hallazgo |
| `crear_nota` | Nota en evento existente |
| `eliminar_evento` | Borrar por id |
| `seek_a_evento` | Seek + snapshot de reproducción |

Evento push: `eventos-qc-actualizados` (registro completo).

## UI

- Workspace **Report**: listado completo, filtros, marcar playhead, notas.
- Panel derecho en **Compare**: vista compacta del mismo componente.
- Franja **timeline-eventos** bajo el transporte: marcadores ▲ por PTS; clic → seek.

## Tests

```bash
cargo test -p diffplayerqc-core eventos
```
