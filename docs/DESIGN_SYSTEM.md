# Sistema de diseño v2

Referencia visual: `Binder1.pdf`. Implementación: `frontend/src/lib/design/tokens.css`.

## Reglas

- Sin gradientes salvo playhead de timeline (M4+).
- Sin sombras suaves; dropdowns con borde 1px sólido.
- `border-radius` máximo 4px (default 2px).
- Títulos de panel: mayúsculas + `letter-spacing: 0.05em`.
- Datos técnicos en fuente mono (timecode, fps, LUFS).
- Iconos: **@lucide/svelte**, stroke 1.5, 16px toolbar / 18px tabs. Sin emoji en UI.

## Workspaces

| ID | Atajo | Layout |
|----|-------|--------|
| compare | Shift+1 | Izq fuentes · centro canvas · der diff/niveles |
| inspect | Shift+2 | Izq fuentes · centro canvas+scopes · der histograma |
| audio | Shift+3 | Izq meta · centro waveforms |
| report | Shift+4 | Placeholder |
| export | Shift+5 | Placeholder |

Persistencia: `localStorage` clave `diffplayerqc-v2-layout` (por workspace).
