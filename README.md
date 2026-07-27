# Rust Testing

Repositorio complementario de Jeresoft Academy para estudiar testing en Rust.
Profundiza el estándar transversal de pruebas de RFC-0001 §13 y lo convierte en
un curso completo para ingenieros de software.

El objetivo no es memorizar macros ni herramientas. El objetivo es aprender a
diseñar software verificable: qué prueba una suite, qué no puede probar, cómo
se detectan huecos, cómo se mide confianza y cuándo una prueba empieza a
mentir.

## Qué contiene

- Capítulos en Markdown compatibles con publicación posterior.
- Modelos Rust pequeños para representar decisiones de prueba.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks cuando una decisión tenga costo observable.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar en el camino

Este curso es complementario. Refuerza todos los repositorios del camino
troncal: algoritmos, estructuras de datos, sistemas, arquitectura, cloud,
DevOps y dominios aplicados.

Recibe fundamentos de Rust, diseño modular y pensamiento de ingeniería. A su
vez alimenta todos los cursos que necesitan pruebas más expresivas:
`rust-cloud`, `rust-devops`, `rust-software-architecture`, `rust-api-design`,
`rust-performance` y `rust-projects`.

## Estado editorial

El curso está en estado `planned`: el repositorio, milestones e issues ya
existen, pero los capítulos aún no están implementados. Esto no significa que
el curso esté publicado. La revisión humana de Joel sigue siendo obligatoria
antes de usar `reviewed` o `published`.

## Capítulos planeados

| # | Capítulo | Módulo sugerido | Estado |
|---|----------|-----------------|--------|
| 01 | Fundamentos de testing | `src/fundamentals.rs` | draft |
| 02 | Unit tests en Rust | `src/unit_tests.rs` | planned |
| 03 | Tests de integración | `src/integration_tests.rs` | planned |
| 04 | Test doubles | `src/test_doubles.rs` | planned |
| 05 | Property-based testing | `src/property_testing.rs` | planned |
| 06 | Contract testing | `src/contract_testing.rs` | planned |
| 07 | Mutation testing | `src/mutation_testing.rs` | planned |
| 08 | Performance testing | `src/performance_testing.rs` | planned |
| 09 | Chaos testing | `src/chaos_testing.rs` | planned |
| 10 | Estrategia de calidad para sistemas reales | `src/quality_strategy.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`. En este repositorio, `planned`
significa que el plan existe como milestones e issues, pero el contenido aún
no está listo para revisión humana.

## Estructura

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
course.manifest.json
docs/
src/
examples/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Ejecutar pruebas:

```bash
cargo test
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
git diff --check
```

Además, el check remoto `rust` en GitHub Actions debe pasar antes de considerar
listo cualquier corte o PR autónomo.

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `course.manifest.json` expone el mapa estructurado del curso para
  `academy-web`.
- `docs/SUMMARY.md` contiene la navegación del curso.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- El plan inicial vive en 11 milestones y 43 issues de GitHub.
- Cada issue accionable debe estar asignado a `jeresoftx`, tener labels
  coherentes y pertenecer al milestone correspondiente.
- Antes de tocar código de curso, debe existir un issue y el PR debe cerrar
  ese issue con `Closes #N`.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. Testing no es
un trámite para subir cobertura: es una forma de diseñar software que puede
explicarse, verificarse y evolucionar sin perder criterio humano.
