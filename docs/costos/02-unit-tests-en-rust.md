# Costos y benchmarks: unit tests en Rust

**Estado:** draft

Este capítulo no agrega benchmark medido. El modelo de `src/unit_tests.rs`
evalúa decisiones de escala y visibilidad; sus operaciones son triviales y no
representan un costo de dominio.

Agregar un benchmark artificial aquí produciría una señal pobre:

- mediría comparaciones de enums y búsqueda en vectores pequeños;
- no enseñaría nada sobre rendimiento real de pruebas unitarias;
- podría reforzar la idea equivocada de que toda sección de benchmarks debe
  contener una medición aunque no exista una pregunta de costo.

La validación correcta para este capítulo es que `cargo bench --all-targets`
siga pasando y que el texto explique por qué no se mide. Los benchmarks reales
aparecerán cuando el curso estudie rendimiento, regresiones y presupuesto de
ejecución.
