# Tests de integración

**Estado:** draft

## Introducción

Las pruebas de integración comprueban que partes diseñadas por separado
mantienen su promesa cuando se usan juntas. En Rust, normalmente viven en
`tests/` y consumen el crate como lo haría otra persona: por su API pública.

Este capítulo estudia esa distancia deliberada. No se trata de repetir cada
unit test desde fuera, sino de obtener evidencia de que los módulos, tipos y
flujos se conectan sin filtrar detalles internos.

## Concepto

Una prueba de integración verifica un contrato que aparece al cruzar una
frontera: entre módulos, entre capas de un crate o entre un adaptador y un
sistema controlado. El sujeto de la prueba es el comportamiento compuesto, no
la implementación privada de una pieza.

En un crate de Rust, un archivo bajo `tests/` se compila como consumidor
externo. Esa condición es valiosa: impide acceder a elementos privados y hace
visible si la API pública permite completar un flujo real.

## Problema

Un sistema puede tener unit tests impecables y aun así fallar al combinar sus
partes. Dos módulos pueden respetar sus reglas locales, pero discrepar sobre el
formato de un dato, el orden de una operación o la forma de reportar un error.

El extremo contrario también es costoso. Una suite que usa una base de datos,
red, reloj real y estado compartido para cada caso se vuelve lenta,
impredecible y difícil de diagnosticar. La intención del capítulo es encontrar
la mínima integración que produzca evidencia honesta.

## Alternativas

La primera alternativa es depender solo de unit tests. Da retroalimentación
rápida y local, pero no demuestra que las piezas colaboren por su contrato
público.

La segunda es probar todos los flujos contra infraestructura real. Se acerca al
entorno de producción, aunque introduce variables ajenas al contrato que se
quiere entender y vuelve la suite más frágil.

La tercera es diseñar pruebas de integración con fronteras explícitas,
dependencias controladas y escenarios completos pero pequeños. El curso adopta
esta alternativa: conserva realismo en el contrato sin convertir cada prueba
en una prueba de sistema.

## Invariantes

- Una prueba de integración usa la API pública que se desea proteger.
- Cada escenario nombra el flujo o contrato compuesto que verifica.
- La infraestructura externa se controla, sustituye o aísla de forma visible.
- Una falla debe acotar una frontera de colaboración, no ocultarse entre ruido
  ambiental.
- Un test de integración no reemplaza la evidencia local de los unit tests.
- La suite debe evitar depender del orden de ejecución, del reloj real o de
  datos compartidos sin una justificación explícita.

## Límites del capítulo

Este capítulo no enseña todavía contratos entre servicios desplegados, test
doubles en profundidad ni pruebas de rendimiento. Esos temas aparecen en los
capítulos de contract testing, test doubles y performance testing.

Aquí la pregunta central es más acotada: ¿qué comportamiento solo se vuelve
observable cuando el consumidor cruza la API pública y coordina más de un
módulo?

## Preparación para el modelo Rust

El modelo mínimo representará la frontera integrada, la superficie desde la
que se observa y los huecos que vuelven poco confiable una prueba. Con esos
datos podrá recomendar una estrategia de entorno y estimar la señal esperada.

No se agregan dependencias externas para esta especificación.

No está marcado como `reviewed` ni `published`.
