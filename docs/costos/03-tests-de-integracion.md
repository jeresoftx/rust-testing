# Costos y benchmarks: tests de integración

**Estado:** draft

Este capítulo no agrega un benchmark medido. El modelo de
`src/integration_tests.rs` clasifica fronteras, entornos y riesgos; medir
comparaciones de enums o recorridos de vectores pequeños no responde una
pregunta de rendimiento útil.

Agregar un benchmark artificial aquí produciría una señal pobre:

- no mediría el costo real de integrar componentes;
- confundiría tiempo de ejecución del modelo pedagógico con tiempo de una suite;
- reforzaría la idea equivocada de que toda sección de benchmarks necesita una
  cifra, aunque no exista una decisión de rendimiento detrás.

La pregunta de costo útil para una prueba de integración depende del sistema:
cuánto tarda el fixture, cuánto cuesta levantar un sandbox y qué presupuesto de
tiempo puede aceptar la retroalimentación del equipo. Esas mediciones requieren
un caso real y aparecerán cuando el curso trate rendimiento, regresiones y
estrategias de calidad para sistemas reales.

La verificación de ruta de este repositorio sigue siendo
`cargo bench --all-targets`.
