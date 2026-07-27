# Estrategia de calidad para sistemas reales

**Estado:** draft

## Introducción

La calidad de un sistema real no sale de una sola técnica. Aparece cuando las
pruebas, revisión, integración continua, observabilidad y criterio humano se
refuerzan sin fingir que alguno reemplaza al resto.

## Concepto

Una estrategia de calidad asigna cada señal a la decisión que puede sostener:
unit tests para reglas locales, integración para colaboración, CI para evitar
regresiones conocidas, observabilidad para el sistema desplegado y revisión
humana para riesgos que la automatización no entiende por sí sola.

## Problema

Acumular herramientas sin una frontera clara produce ruido, duplicación y falsa
seguridad. Confiar solo en revisión humana no escala; confiar solo en una suite
verde ignora datos, operación y cambios de intención.

## Alternativas

Una pirámide rígida puede ser útil como imagen, pero no decide por dominio.
Medir cobertura como objetivo simplifica el reporte, pero puede ocultar huecos.
El capítulo adopta una estrategia por riesgo y señal: cada técnica debe tener
una pregunta explícita que responder.

## Invariantes

- Cada señal de calidad declara qué riesgo reduce y qué no puede demostrar.
- Un fallo en producción alimenta nuevas pruebas, alertas o decisiones de diseño.
- Una suite verde no equivale a aprobación humana ni a salud operativa.
- La estrategia evoluciona con el sistema y sus riesgos reales.
- La automatización informa; el criterio humano decide.

## Límites del capítulo

No prescribe una herramienta de CI ni una plataforma de observabilidad. Integra
los conceptos del curso en una forma de decidir, revisar y aprender.

## Preparación para el modelo Rust

El modelo representará una señal, el riesgo que reduce y sus límites, sin
agregar dependencias externas.

No está marcado como `reviewed` ni `published`.
