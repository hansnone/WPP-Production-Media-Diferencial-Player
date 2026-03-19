# Perfiles de compilación

## `release` (por defecto)

- **`opt-level = 3`**: prioriza rendimiento en CPU (decodificación, copias, UI). Recomendado para sesiones de QC en máquina de trabajo.
- **`lto = true`**, **`codegen-units = 1`**: mejora de optimización a costa de tiempos de link más largos.
- **`strip = true`**, **`panic = "abort"`**: binario más compacto y sin unwinding en pánico.

```bash
cargo build --release
```

## `release-small`

Hereda `release` pero usa **`opt-level = "s"`** (optimizar tamaño). Útil si el objetivo principal es empaquetado/portabilidad y se acepta algo menos de velocidad en rutas calientes.

```bash
cargo build --profile release-small
```

## Trade-off resumido

| Perfil           | Tamaño binario | Velocidad CPU típica |
|------------------|----------------|----------------------|
| `release`        | Mayor          | Mejor                |
| `release-small`  | Menor          | Menor                |

Para comparación A/B con dos decoders y `sws_scale`, **`release`** suele notarse más que el ahorro de tamaño de `release-small`.
