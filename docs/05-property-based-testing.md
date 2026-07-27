# Property-based testing

**Estado:** draft

## Introducción

Property-based testing comprueba una regla que debe mantenerse para muchos
datos, en vez de afirmar solo unos cuantos ejemplos elegidos a mano. Su valor
no es generar casos al azar: es obligar a declarar una propiedad del dominio y
conservar el contraejemplo más pequeño que la contradice.

## Concepto

Una propiedad relaciona entradas y resultados. Por ejemplo, normalizar texto
dos veces debe dar el mismo resultado que normalizarlo una sola vez. Un
generador propone entradas dentro de un dominio; shrinking reduce una falla a
un caso que se pueda entender y convertir en ejemplo de regresión.

## Problema

Los ejemplos concretos cubren historias conocidas, pero suelen omitir
combinaciones, bordes y datos inesperados. Al mismo tiempo, una propiedad vaga
puede producir muchos casos sin enseñar nada: "no debe fallar" no describe un
contrato.

## Alternativas

Probar solo ejemplos hace la suite fácil de leer, pero deja espacio para
contraejemplos no imaginados. Generar datos sin propiedad aumenta volumen sin
evidencia. El capítulo adopta propiedades pequeñas, dominios explícitos y
contraejemplos reproducibles, complementando en vez de reemplazar ejemplos.

## Invariantes

- Una propiedad debe expresar una relación observable del dominio.
- El generador debe declarar el dominio que representa.
- Un contraejemplo se conserva como información, no como ruido aleatorio.
- Shrinking busca claridad diagnóstica, no ocultar la falla.
- Las propiedades no sustituyen ejemplos de negocio ni tests de integración.

## Límites del capítulo

No se introduce un framework externo ni se promete probar todos los valores
posibles. El objetivo es aprender a formular propiedades, razonar sobre sus
dominios y leer contraejemplos honestos.

## Preparación para el modelo Rust

El modelo representará la clase de propiedad, el dominio de generación y los
riesgos que debilitan la señal. No se agregan dependencias externas.

No está marcado como `reviewed` ni `published`.
