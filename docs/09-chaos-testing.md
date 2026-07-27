# Chaos testing

**Estado:** draft

## Introducción

Chaos testing introduce una falla controlada para comprobar una hipótesis sobre
el comportamiento del sistema. No busca romper por espectáculo: busca reducir
incertidumbre antes de que una falla real elija el momento y las condiciones.

## Concepto

Un experimento declara alcance, hipótesis, inyección de falla y señal esperada.
Puede simular una dependencia lenta, una respuesta inválida o una interrupción
acotada. El resultado sirve para aprender si el sistema degrada, se recupera o
expone una fragilidad.

## Problema

Los flujos felices no demuestran recuperación. Pero inyectar fallas sin límite
ni observación puede causar daño y producir conclusiones confusas. La técnica
necesita guardas, reversibilidad y una pregunta operativa concreta.

## Alternativas

Esperar incidentes reales enseña tarde. Simular todo en unit tests pierde el
contexto operacional. El capítulo adopta experimentos pequeños, controlados y
reversibles que complementan pruebas locales y observabilidad.

## Invariantes

- Todo experimento declara hipótesis y señal de recuperación.
- El radio de impacto está acotado antes de inyectar la falla.
- Existe una forma explícita de detener o revertir el experimento.
- Un resultado inesperado se investiga, no se normaliza.
- Ningún ejercicio autoriza afectar producción.

## Límites del capítulo

No despliega herramientas de caos ni ejecuta fallas contra servicios reales. Se
concentra en el criterio para diseñar experimentos seguros y legibles.

## Preparación para el modelo Rust

El modelo representará la hipótesis, el alcance, el tipo de falla y los riesgos
que impiden interpretar un experimento con confianza.

No está marcado como `reviewed` ni `published`.
