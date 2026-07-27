# Costos y benchmarks: fundamentos de testing

**Estado:** draft

Este capítulo no incluye benchmark medido porque el modelo de
`src/fundamentals.rs` no representa una decisión de rendimiento. Construir un
benchmark aquí daría una señal falsa: mediría operaciones triviales sobre
enumeraciones y vectores pequeños, no una propiedad relevante del curso.

La decisión educativa es explícita:

- sí se ejecuta `cargo bench --all-targets` para verificar que el repositorio
  conserva la ruta de benchmarks limpia;
- no se agrega un benchmark artificial para `TestClaim`;
- los benchmarks con valor aparecerán cuando exista una pregunta real de costo,
  tiempo, carga o regresión de rendimiento.

Esta postura protege una regla del curso: una métrica es señal, no objetivo.
Medir por medir puede ser tan engañoso como subir cobertura sin expectativas
fuertes.
