# Ejercicios: property-based testing

**Estado:** draft

## Nivel 1: formular propiedades

Clasifica como idempotencia, round trip o invariante: normalizar texto,
serializar y leer un valor, y ordenar una colección sin cambiar su longitud.

```bash
cargo run --example property_testing_nivel_1
```

## Nivel 2: delimitar el dominio

Para cada propiedad decide si el dominio debe ser valores válidos, valores
acotados o casos enfocados en bordes. Marca una afirmación vaga y explica por
qué no ayuda a generar contraejemplos.

```bash
cargo run --example property_testing_nivel_2
```

## Nivel 3: conservar el contraejemplo

Diseña la propiedad de registro y lectura de una preferencia. Declara cómo el
caso reducido se convertiría en una regresión reproducible.

```bash
cargo run --example property_testing_nivel_3
```
