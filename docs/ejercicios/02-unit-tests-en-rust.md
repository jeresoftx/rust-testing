# Ejercicios: unit tests en Rust

**Estado:** draft

Estos ejercicios practican la decisión central del capítulo: no toda regla
pequeña debe probarse de la misma forma. La ubicación de una prueba importa,
pero la señal importa más.

## Nivel 1: elegir la escala

Clasifica cada regla como unit test, doctest o integración:

1. "normaliza un correo antes de comparar dominios";
2. "parsea una ruta pública desde texto";
3. "sincroniza carrito y catálogo";
4. "rechaza un precio negativo antes de guardar".

Solución ejecutable:

```bash
cargo run --example unit_tests_nivel_1
```

## Nivel 2: reconocer huecos

Toma estas pruebas hipotéticas y nombra el hueco principal:

1. verifica el nombre de una función privada;
2. solo cubre una edad válida, pero no el mínimo ni el máximo;
3. intenta probar desde un módulo una regla que requiere base de datos y API;
4. documenta una API pública con un ejemplo que no afirma comportamiento.

Solución ejecutable:

```bash
cargo run --example unit_tests_nivel_2
```

## Nivel 3: diseñar sin exponer de más

Diseña una suite mínima para un módulo de validación de usuarios:

1. email vacío se rechaza;
2. email sin arroba se rechaza;
3. email válido produce un tipo normalizado;
4. el tipo público se puede construir desde texto.

Tu objetivo es separar qué pertenece a unit tests internos y qué debe quedar
como doctest público. No hagas pública una función solo para probarla.

Solución ejecutable:

```bash
cargo run --example unit_tests_nivel_3
```

## Nivel 4: extensión sin solución canónica

Elige un módulo real de un proyecto propio y responde:

- ¿qué reglas pequeñas necesitan unit tests cerca del código?
- ¿qué ejemplo público merece doctest?
- ¿qué escenario ya cruza módulos y debe salir a integración?

La revisión humana debe evaluar si las pruebas protegen comportamiento o si
solo inmovilizan detalles de implementación.
