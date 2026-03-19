# Contribuir — Comentarios y documentación (español)

## Política de comentarios

**No** se exige comentario en cada línea: genera ruido y dificulta el mantenimiento.

### Qué sí hacer

1. **`//!` (documentación de módulo)** al inicio de cada archivo `.rs`: qué problema resuelve, qué hilos toca, dependencias importantes.
2. **`///` (rustdoc)** en todo lo **público** (`pub fn`, `pub struct`, `pub enum`): parámetros, errores posibles, invariantes.
3. **Bloques `//`** antes de lógica delicada: diferimiento de entrada en macOS, cálculo de PTS, drenado de canales, condiciones de carrera.

### Qué evitar

- Comentar imports, getters triviales o código que ya se lee claro en Rust.
- Duplicar en comentario lo que el nombre del símbolo ya dice.

### Idioma

Comentarios y rustdoc del proyecto: **español**, salvo términos técnicos habituales en inglés (PTS, shader, frame).

## Formato

- `cargo fmt` antes de commit.
- `cargo clippy` sin warnings nuevos cuando sea posible.
