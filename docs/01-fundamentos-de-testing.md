# Fundamentos de testing

**Estado:** draft

Este capítulo abre el curso con una pregunta que parece simple y no lo es:
¿qué prueba realmente una prueba?

Testing no es una lista de macros ni un trámite para subir cobertura. Testing
es una forma de escribir evidencia sobre el comportamiento esperado de un
sistema. Esa evidencia no reemplaza el criterio humano, pero reduce el espacio
donde los cambios pueden romper algo sin ser vistos.

## Concepto

Una prueba es una observación controlada sobre un comportamiento. Toma una
entrada, prepara un contexto, ejecuta una acción y compara el resultado con una
expectativa. Lo importante no es solo que la comparación pase, sino que la
expectativa represente una regla valiosa del sistema.

Por eso una buena prueba comunica tres cosas:

- qué comportamiento importa;
- bajo qué condiciones debe sostenerse;
- qué señal aparece cuando se rompe.

Una suite de pruebas es el conjunto de esas observaciones. No demuestra que el
sistema sea perfecto; demuestra que ciertas propiedades siguen siendo ciertas
en los casos que decidimos volver ejecutables.

## Problema

El software cambia. Cada cambio puede romper una regla anterior, introducir una
regla nueva o revelar que nunca entendimos bien una frontera. Sin pruebas, la
confianza depende de memoria, intuición y revisión manual completa cada vez.

Ese modelo no escala. Un sistema real acumula casos límite, invariantes,
contratos entre módulos y decisiones históricas. Si esas decisiones solo viven
en la cabeza de alguien, se pierden o se reinterpretan. Las pruebas convierten
parte de esa memoria en evidencia ejecutable.

## Qué puede probar una prueba

Una prueba puede verificar que una expectativa concreta se cumple en un
escenario definido. Puede hacer visibles regresiones, contratos rotos, errores
de borde y decisiones de diseño que alguien intentó cambiar sin querer.

También puede documentar comportamiento. Un lector que ve una prueba bien
nombrada entiende qué caso le importó al autor y por qué esa regla merece
protección.

## Qué no puede probar una prueba

Una prueba no demuestra ausencia total de errores. Tampoco demuestra que el
diseño sea correcto, que el producto resuelva el problema correcto o que la
suite cubra todos los escenarios relevantes.

La cobertura de líneas tampoco equivale a confianza. Puede indicar que el
código se ejecutó, pero no que las expectativas sean fuertes. Una suite puede
tener cobertura alta y aun así no detectar mutaciones triviales, contratos
ambiguos o decisiones de negocio mal entendidas.

## Alternativas

La primera alternativa es confiar en pruebas manuales. Sirven para exploración,
criterio de producto y revisión humana, pero son costosas de repetir y fáciles
de olvidar.

La segunda alternativa es probar solo al final. Esto detecta errores tarde,
cuando el costo de entender la causa ya subió y el diseño quizá quedó rígido.

La tercera alternativa es perseguir cobertura como objetivo. Da una métrica
visible, pero puede convertir la suite en teatro: mucho código ejecutado y poca
confianza real.

La alternativa que toma este curso es tratar las pruebas como diseño
ejecutable. Primero se entiende la regla, luego se decide qué evidencia la
protege y finalmente se implementa una prueba que pueda fallar por una razón
clara.

## Invariantes del curso

- Una prueba debe proteger una regla observable.
- Una prueba debe fallar por una razón comprensible.
- Una prueba debe tener un nombre que explique el comportamiento, no el detalle
  interno.
- Una prueba debe minimizar causas accidentales de falla.
- Una suite debe combinar escalas: unidad, integración, contrato, propiedades y
  rendimiento cuando aplique.
- Una métrica de testing es señal, no objetivo.
- La revisión humana sigue decidiendo si la evidencia es suficiente.

## Preparación para el modelo Rust

El modelo mínimo del capítulo representará afirmaciones de prueba, tipos de
evidencia y señales de confianza. No usará frameworks externos: el objetivo es
hacer visible el razonamiento antes de conectar herramientas.

Ese modelo debe permitir expresar preguntas como:

- ¿qué comportamiento se afirma?
- ¿qué tipo de evidencia lo protege?
- ¿qué riesgo queda fuera?
- ¿la señal es fuerte o solo cosmética?

## Fuera de alcance

Este capítulo no enseña todavía `#[test]`, mocks, property testing, contratos ni
benchmarks. Esos temas tienen capítulos propios. Aquí se establece el lenguaje
común que hará que esos capítulos no parezcan técnicas aisladas.

No está marcado como `reviewed` ni `published`.
