# AGENTS.md

Este repositorio es parte de la colección complementaria de Jeresoft Academy y
se rige por la RFC-0001 (manual fundacional).

## Objetivo

Crear el mejor recurso educativo posible sobre testing en Rust.

Todo cambio debe mejorar simultáneamente:

- calidad técnica
- claridad
- documentación
- mantenibilidad

## Antes de escribir código

Siempre, en este orden (RFC-0001 §2 y §13):

1. Explicar el concepto.
2. Explicar el problema.
3. Comparar alternativas.
4. Justificar la implementación.

## Código

Conforme a RFC-0001 §13:

- Rust idiomático.
- Clippy limpio y rustfmt sin diffs.
- Sin `unsafe` salvo justificación documentada con comentario `SAFETY`.
- Comentarios solo donde aporten valor.
- Dependencias externas solo con justificación escrita.

## Documentación

Todo capítulo sigue las doce secciones de RFC-0001 §14 y la plantilla de §16.
Toda nueva funcionalidad incluye:

- README o ROADMAP actualizado cuando aplique.
- Diagramas Mermaid cuando ayuden a razonar.
- Ejemplos ejecutables.
- Tests.
- Benchmarks si aplican; si no aplican, se declara por qué.

## GitHub

El plan inicial del curso vive como milestones e issues. Cada issue accionable
debe estar asignado a `jeresoftx`, tener labels coherentes y pertenecer al
milestone del capítulo o fase correspondiente.

Cada PR debe:

- resolver un solo issue;
- tener un solo commit principal;
- incluir `Closes #N`;
- conservar milestone, asignación y labels del issue;
- pasar las verificaciones aplicables antes de fusionarse.

## Nunca

- Agregar dependencias innecesarias.
- Optimizar prematuramente.
- Duplicar código.
- Omitir documentación.
- Publicar capítulos parciales.
- Marcar contenido como `reviewed` o `published` sin revisión humana explícita.

## Filosofía

Este repositorio debe poder utilizarse como un libro de ingeniería. Nunca
sacrificar claridad por ingenio. Explicar el porqué, no solo el cómo.
