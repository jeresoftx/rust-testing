use rust_testing::contract_testing::{
    Compatibility, ContractDecision, ContractDirection, ContractGap,
};
fn main() -> Result<(), rust_testing::contract_testing::ContractError> {
    let c = ContractDecision::new(
        "reserva inventario y reporta falta de stock",
        ContractDirection::ConsumerToProvider,
        Compatibility::BackwardCompatible,
    )?
    .with_gap(ContractGap::MissingErrorCase);
    println!("{} => señal: {:?}", c.operation(), c.signal());
    Ok(())
}
