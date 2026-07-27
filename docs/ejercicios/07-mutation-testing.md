# Ejercicios: mutation testing

**Estado:** draft

## Nivel 1: leer resultados

Clasifica un mutante muerto, uno sobreviviente y uno equivalente. Explica qué
resultado requiere una investigación adicional.

```bash
cargo run --example mutation_testing_nivel_1
```

## Nivel 2: distinguir cobertura de evidencia

Parte de una condición ejecutada por la suite pero sin aserción de resultado.
Registra por qué esa cobertura no mata necesariamente un mutante.

```bash
cargo run --example mutation_testing_nivel_2
```

## Nivel 3: convertir un superviviente en regresión

Para un descuento con límite, identifica un mutante relevante y formula la
aserción de comportamiento que debería eliminarlo.

```bash
cargo run --example mutation_testing_nivel_3
```
