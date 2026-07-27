use rust_testing::contract_testing::{
    Compatibility, ContractDecision, ContractDirection, ContractGap,
};
fn main() -> Result<(), rust_testing::contract_testing::ContractError> {
    let c = ContractDecision::new(
        "reserva inventario",
        ContractDirection::ProviderToConsumer,
        Compatibility::CoordinatedChange,
    )?
    .with_gap(ContractGap::UnversionedBreakingChange);
    println!(
        "{} => {:?}, señal: {:?}",
        c.operation(),
        c.compatibility(),
        c.signal()
    );
    Ok(())
}
