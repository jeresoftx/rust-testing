use rust_testing::contract_testing::{
    Compatibility, ContractDecision, ContractDirection, ContractGap, ContractSignal,
};
#[test]
fn consumer_can_observe_error_gap() {
    let d = ContractDecision::new(
        "consulta saldo",
        ContractDirection::ConsumerToProvider,
        Compatibility::BackwardCompatible,
    )
    .expect("not empty")
    .with_gap(ContractGap::MissingErrorCase);
    assert_eq!(d.signal(), ContractSignal::Focused);
}
