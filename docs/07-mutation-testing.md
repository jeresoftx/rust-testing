# Mutation testing

**Estado:** draft

## Introducción

Mutation testing modifica deliberadamente una regla pequeña y pregunta si la
suite nota el cambio. Un mutante que sobrevive no es una calificación del
equipo: es una pista de que falta una expectativa observable o que el mutante
es equivalente al comportamiento original.

## Concepto

Un mutante cambia una condición, un operador o una respuesta. La suite lo mata
si falla; sobrevive si todos los tests siguen pasando. El resultado sirve para
examinar la fuerza de la evidencia, no para perseguir una cifra aislada.

## Problema

La cobertura puede ejecutar una línea sin afirmar su regla. Una suite extensa
puede dejar sobrevivir cambios relevantes. El extremo opuesto es forzar una
cifra de mutación sin investigar equivalencias ni costo de diagnóstico.

## Alternativas

Confiar solo en cobertura mide ejecución. Revisar manualmente cada línea da
criterio, pero escala mal. El capítulo combina mutantes pequeños, investigación
de supervivientes y ejemplos de regresión cuando la regla sí importa.

## Invariantes

- Un mutante sobreviviente se investiga antes de agregar una prueba.
- Un mutante equivalente no exige una prueba artificial.
- La nueva prueba debe afirmar comportamiento, no la forma del código.
- La tasa de mutación orienta conversación, no reemplaza criterio humano.

## Límites del capítulo

No instala una herramienta de mutación. Se enfoca en el razonamiento previo:
qué cambio importa, qué evidencia debería detectarlo y cuándo un mutante no
representa una diferencia observable.

## Preparación para el modelo Rust

El modelo describirá el tipo de mutación, su resultado y los riesgos de
interpretar la supervivencia sin contexto.

No está marcado como `reviewed` ni `published`.
