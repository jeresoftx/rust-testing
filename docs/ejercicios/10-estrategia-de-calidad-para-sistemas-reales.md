# Ejercicios: estrategia de calidad para sistemas reales

**Estado:** draft

## Nivel 1: asignar señales

Relaciona una regla de precio, una colaboración entre módulos, una regresión
conocida, un error de producción y una decisión de producto con la señal más
adecuada.

```bash
cargo run --example quality_strategy_nivel_1
```

## Nivel 2: reconocer límites

Identifica por qué CI no equivale a aprobación humana y por qué una suite verde
no reemplaza retroalimentación operativa.

```bash
cargo run --example quality_strategy_nivel_2
```

## Nivel 3: cerrar el ciclo de aprendizaje

Parte de un error detectado en producción y decide qué prueba, señal operativa
o revisión de diseño debe agregarse para reducir el riesgo en el futuro.

```bash
cargo run --example quality_strategy_nivel_3
```
