# Contract testing

**Estado:** draft

## Introducción

Contract testing verifica una promesa compartida entre quien consume una API y
quien la provee. No prueba toda la infraestructura: protege las expectativas
que cruzan ese límite y hace visible cuándo una evolución deja de ser
compatible.

## Concepto

Un contrato describe solicitudes, respuestas, errores y reglas de
compatibilidad observables. El consumidor declara qué necesita; el proveedor
verifica que puede cumplirlo. La evidencia está en la promesa compartida, no en
la implementación interna de ninguno.

## Problema

Dos servicios pueden pasar sus suites locales y fallar al desplegarse juntos:
un campo cambió de significado, un error dejó de existir o una respuesta ya no
es aceptada. Probar cada combinación en un entorno completo es caro y lento,
pero no probar el límite deja la incompatibilidad para producción.

## Alternativas

Solo usar integración de extremo a extremo da realismo, pero diagnóstico lento.
Usar mocks internos da control, pero puede inventar un proveedor ficticio. El
capítulo adopta contratos explícitos y pequeños que complementan integración y
prueban compatibilidad antes del despliegue.

## Invariantes

- El contrato nombra consumidor, proveedor y operación observable.
- Un cambio compatible conserva expectativas existentes.
- Un cambio incompatible se declara, versiona o coordina explícitamente.
- El contrato no expone detalles internos sin valor para el consumidor.
- Los errores esperados también son parte del contrato.

## Límites del capítulo

No enseña un broker ni un framework específico. Se concentra en formular el
contrato y razonar sobre compatibilidad sin agregar dependencias externas.

## Preparación para el modelo Rust

El modelo representará la dirección del contrato, su compatibilidad y los
riesgos que reducen la confianza de la verificación.

No está marcado como `reviewed` ni `published`.
