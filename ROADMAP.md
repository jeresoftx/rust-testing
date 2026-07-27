# Roadmap

**Curso:** rust-testing  
**Estado actual:** planned  
**Tipo:** complementario técnico  
**Fuente:** RFC-0001 §10, §13, §14 y §15

Este roadmap registra el avance del curso sin convertirlo en fecha límite. El
camino importa más que cerrar rápido: cada capítulo avanza cuando puede cumplir
el estándar de Jeresoft Academy.

## Capítulos

| # | Capítulo | Milestone | Estado |
|---|----------|-----------|--------|
| 01 | Fundamentos de testing | 01. Fundamentos de testing | draft |
| 02 | Unit tests en Rust | 02. Unit tests en Rust | planned |
| 03 | Tests de integración | 03. Tests de integración | planned |
| 04 | Test doubles | 04. Test doubles | planned |
| 05 | Property-based testing | 05. Property-based testing | planned |
| 06 | Contract testing | 06. Contract testing | planned |
| 07 | Mutation testing | 07. Mutation testing | planned |
| 08 | Performance testing | 08. Performance testing | planned |
| 09 | Chaos testing | 09. Chaos testing | planned |
| 10 | Estrategia de calidad para sistemas reales | 10. Estrategia de calidad para sistemas reales | planned |

## Cierre editorial

El milestone `11. Cierre editorial y publicación` cubre navegación, estados,
verificaciones y publicación candidata interna. Ningún capítulo pasa a
`reviewed` o `published` por automatización.

## Flujo de trabajo

Cada capítulo se divide en cuatro pasos:

1. Especificar concepto, problema e invariantes.
2. Implementar modelo Rust mínimo.
3. Escribir capítulo, diagrama y ejemplos.
4. Agregar ejercicios, soluciones y benchmarks.

Cada paso vive como issue de GitHub antes de tocar el código correspondiente.
