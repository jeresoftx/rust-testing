# Ejercicios: contract testing

**Estado:** draft

## Nivel 1: nombrar el contrato

Define consumidor, proveedor y operación para consultar saldo, crear un pedido
y registrar un pago. Indica la dirección de cada contrato.

```bash
cargo run --example contract_testing_nivel_1
```

## Nivel 2: reconocer compatibilidad

Clasifica cambios como compatibles o coordinados: agregar un campo opcional,
eliminar un error esperado y cambiar el significado de un campo existente.

```bash
cargo run --example contract_testing_nivel_2
```

## Nivel 3: proteger el error observable

Diseña el contrato de reserva de inventario. Incluye el éxito, falta de stock y
la decisión de versionar cualquier cambio incompatible.

```bash
cargo run --example contract_testing_nivel_3
```
