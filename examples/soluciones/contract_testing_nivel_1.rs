use rust_testing::contract_testing::{Compatibility, ContractDecision, ContractDirection};
fn main() -> Result<(), rust_testing::contract_testing::ContractError> {
    for c in [
        ContractDecision::new(
            "consulta saldo",
            ContractDirection::ConsumerToProvider,
            Compatibility::BackwardCompatible,
        )?,
        ContractDecision::new(
            "crea pedido",
            ContractDirection::ConsumerToProvider,
            Compatibility::BackwardCompatible,
        )?,
    ] {
        println!("{} => {:?}", c.operation(), c.direction());
    }
    Ok(())
}
