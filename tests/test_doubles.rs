use rust_testing::test_doubles::{
    DoubleContract, DoubleDecision, DoubleGap, DoubleKind, DoubleSignal,
};

#[test]
fn consumer_can_observe_a_mock_without_internal_access() {
    let decision = DoubleDecision::new(
        "publicador de eventos",
        DoubleKind::Mock,
        DoubleContract::Interaction,
    )
    .expect("collaboration is not empty")
    .with_gap(DoubleGap::ImplementationCoupling);
    assert_eq!(decision.signal(), DoubleSignal::Weak);
}
