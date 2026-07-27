# Candidata interna de revisión

**Estado:** draft

Esta candidata reúne los diez capítulos técnicos, modelos Rust, ejemplos,
ejercicios y verificaciones del curso `rust-testing`. Es una referencia para la
revisión humana; no es una publicación ni cambia el estado editorial de los
capítulos.

## Alcance de la candidata

- Diez capítulos técnicos navegables desde `docs/SUMMARY.md`.
- Modelos Rust con pruebas unitarias, de integración y doctests.
- Ejemplos ejecutables y soluciones de ejercicios de niveles 1 a 3.
- Notas por capítulo sobre costos y benchmarks aplicables.
- Diagramas Mermaid para razonar sobre decisiones de testing.

## Verificación técnica requerida

Antes de una revisión humana, la candidata debe pasar:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
git diff --check
```

El check remoto `rust` de GitHub Actions también debe estar en verde.

## Revisión humana pendiente

La revisión debe evaluar claridad pedagógica, precisión técnica, progresión de
ejemplos, calidad de ejercicios y coherencia con RFC-0001. Las correcciones se
registran como nuevos issues y PRs.

Ningún capítulo está marcado como `reviewed` ni `published`. Esos estados solo
pueden cambiar mediante decisión humana explícita.
