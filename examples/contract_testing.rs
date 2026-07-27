use rust_testing::contract_testing::{
    Compatibility, ContractDecision, ContractDirection, ContractGap,
};

fn main() -> Result<(), rust_testing::contract_testing::ContractError> {
    let compatible = ContractDecision::new(
        "consulta saldo",
        ContractDirection::ConsumerToProvider,
        Compatibility::BackwardCompatible,
    )?;
    let breaking = ContractDecision::new(
        "crea pedido",
        ContractDirection::ProviderToConsumer,
        Compatibility::CoordinatedChange,
    )?
    .with_gap(ContractGap::UnversionedBreakingChange);

    for contract in [compatible, breaking] {
        println!(
            "{} => compatibilidad: {:?}, señal: {:?}, huecos: {:?}",
            contract.operation(),
            contract.compatibility(),
            contract.signal(),
            contract.gaps()
        );
    }
    Ok(())
}
