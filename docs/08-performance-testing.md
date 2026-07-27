# Performance testing

**Estado:** draft

## Introducción

Performance testing responde una pregunta de costo con una medición diseñada
para esa pregunta. Un benchmark no demuestra que producción sea rápida: ofrece
evidencia local bajo condiciones explícitas y ayuda a detectar regresiones.

## Concepto

Una medición compara un escenario, una carga y un presupuesto. El resultado
debe separar señal de ruido y registrar qué se midió. Una regresión es un cambio
relevante respecto a una referencia, no una variación aislada de una ejecución.

## Problema

Medir una operación trivial sin decisión asociada crea números atractivos pero
inútiles. A su vez, concluir capacidad de producción desde una laptop ignora
red, datos, concurrencia y despliegue.

## Alternativas

No medir deja las regresiones ocultas. Medir todo sin hipótesis añade ruido. El
capítulo adopta escenarios representativos, presupuestos explícitos y lectura
prudente de resultados, complementada por observabilidad en producción.

## Invariantes

- Toda medición declara escenario, unidad y presupuesto.
- Una variación se compara contra una referencia antes de llamarse regresión.
- El entorno de medición se documenta cuando afecta la lectura.
- Un benchmark local no se presenta como capacidad de producción.

## Límites del capítulo

No configura infraestructura de carga ni observabilidad distribuida. Prepara el
criterio para decidir qué medir, cómo interpretar ruido y cuándo investigar.

## Preparación para el modelo Rust

El modelo describirá el escenario, el presupuesto y el resultado de una
medición sin agregar dependencias externas.

No está marcado como `reviewed` ni `published`.
