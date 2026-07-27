# Ejercicios: tests de integración

**Estado:** draft

Estos ejercicios practican la decisión central del capítulo: una prueba de
integración debe cruzar la menor frontera que permita observar el contrato que
importa al consumidor.

## Nivel 1: identificar la frontera

Clasifica cada escenario según su frontera principal:

1. normaliza una dirección y calcula el envío;
2. crea un pedido y reserva inventario por una API pública;
3. traduce la respuesta de un proveedor de pagos con datos grabados;
4. consulta un servicio de pagos real durante cada ejecución.

Para cada caso, decide si el entorno recomendado es en proceso, fixture o
sandbox. Observa que el cuarto escenario contiene un riesgo aun cuando la
frontera sea válida.

Solución ejecutable:

```bash
cargo run --example integration_tests_nivel_1
```

## Nivel 2: reconocer riesgos de señal

Toma estos escenarios y nombra el riesgo principal:

1. dos pruebas reutilizan el mismo usuario persistido;
2. un flujo depende de la hora real para decidir si vence una factura;
3. un adaptador llama por red a un servicio que no controla la suite;
4. un pedido se confirma, pero nunca se verifica qué ocurre si no hay stock.

Explica cómo el riesgo afecta el diagnóstico de una falla y qué control mínimo
debería agregarse.

Solución ejecutable:

```bash
cargo run --example integration_tests_nivel_2
```

## Nivel 3: diseñar un flujo público pequeño

Diseña la prueba de integración de un flujo de registro:

1. el consumidor envía un correo válido por la API pública;
2. el sistema crea el usuario y prepara el mensaje de bienvenida;
3. un correo duplicado no debe producir un segundo usuario;
4. el escenario debe ejecutarse sin red, reloj real ni datos compartidos.

Describe la frontera, la superficie y el entorno. Después agrega un riesgo de
forma intencional y explica cómo cambia la señal esperada.

Solución ejecutable:

```bash
cargo run --example integration_tests_nivel_3
```

## Nivel 4: extensión sin solución canónica

Elige un flujo de un proyecto propio. Define el contrato que un consumidor
debe poder completar, dibuja la frontera que cruza y contesta:

- ¿qué parte sigue perteneciendo a un unit test?
- ¿qué dependencia externa necesita fixture o sandbox?
- ¿qué estado compartido o entrada no determinista puede volver frágil la
  suite?

La revisión humana debe evaluar si el escenario protege una colaboración real o
si usa infraestructura adicional sin aportar evidencia.
