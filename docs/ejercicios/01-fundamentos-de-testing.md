# Ejercicios: fundamentos de testing

**Estado:** draft

Estos ejercicios practican una idea central del capítulo: una prueba no vale
por existir, vale por la evidencia que produce y por los huecos que todavía
deja visibles.

## Nivel 1: nombrar la evidencia

Lee las siguientes reglas y elige el tipo de evidencia más razonable:

1. "rechaza una edad menor a 18 para una cuenta adulta";
2. "serializar y deserializar conserva el mismo valor";
3. "el cliente público no recibe campos internos del proveedor";
4. "el listado responde dentro del presupuesto esperado".

Después crea una afirmación `TestClaim` para cada regla.

Solución ejecutable:

```bash
cargo run --example fundamentos_nivel_1
```

## Nivel 2: detectar huecos de confianza

Toma estas pruebas hipotéticas y marca el hueco principal:

1. la prueba solo llama una función, pero no compara resultado;
2. la prueba depende de la hora real del sistema;
3. la prueba verifica el nombre de una función privada;
4. la prueba cubre el caso feliz, pero no el mínimo permitido.

Después observa cómo cambia la señal de confianza cuando el hueco se registra.

Solución ejecutable:

```bash
cargo run --example fundamentos_nivel_2
```

## Nivel 3: decidir si la señal alcanza

Diseña una suite mínima para proteger el inicio de sesión de una aplicación:

1. credenciales válidas crean sesión;
2. contraseña corta se rechaza;
3. cliente y API sostienen el mismo contrato público;
4. el flujo completo depende de red externa inestable.

Tu tarea no es obtener la señal más alta en todo. Tu tarea es explicar qué
parte de la suite da confianza local, qué parte da confianza de comportamiento
y qué parte necesita rediseño para no depender de azar.

Solución ejecutable:

```bash
cargo run --example fundamentos_nivel_3
```

## Nivel 4: extensión sin solución canónica

Elige una funcionalidad real de un proyecto propio y escribe tres afirmaciones:

- una que proteja una regla local;
- una que proteja una frontera;
- una que revele un hueco todavía aceptado.

No hay solución única. La revisión humana debe evaluar si las reglas son
observables y si los huecos están nombrados con honestidad.
