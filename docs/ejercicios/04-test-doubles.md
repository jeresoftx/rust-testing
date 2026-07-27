# Ejercicios: test doubles

**Estado:** draft

## Nivel 1: elegir el doble

Elige entre stub, fake y mock para: una hora fija, un repositorio temporal y
una publicación de evento que forma parte del contrato.

```bash
cargo run --example test_doubles_nivel_1
```

## Nivel 2: detectar señal débil

Identifica el riesgo principal cuando un fake acepta reglas de negocio distintas
o cuando un mock verifica cada llamada interna de un flujo.

```bash
cargo run --example test_doubles_nivel_2
```

## Nivel 3: diseñar un doble mínimo

Para un registro de usuario, usa un fake para guardar usuarios y un mock solo
para la publicación del evento de bienvenida. Explica qué interacción es
observable y qué detalle interno no debe verificarse.

```bash
cargo run --example test_doubles_nivel_3
```

## Nivel 4: extensión sin solución canónica

Elige una dependencia de un proyecto propio y documenta qué contrato conserva
el doble, qué riesgo introduce y qué prueba de integración sigue siendo
necesaria.
