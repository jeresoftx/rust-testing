# Ejercicios: chaos testing

**Estado:** draft

## Nivel 1: formular hipótesis

Define una hipótesis observable ante latencia, dependencia no disponible y
respuesta inválida. Acota el radio de impacto de cada experimento.

```bash
cargo run --example chaos_testing_nivel_1
```

## Nivel 2: reconocer guardas faltantes

Identifica por qué un experimento sin condición de detención o con alcance no
acotado es inseguro, aunque su hipótesis parezca razonable.

```bash
cargo run --example chaos_testing_nivel_2
```

## Nivel 3: investigar degradación

Diseña un experimento local para una respuesta inválida. Declara qué señal
demuestra degradación controlada y qué resultado obligaría a detenerse.

```bash
cargo run --example chaos_testing_nivel_3
```
