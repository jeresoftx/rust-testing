use rust_testing::integration_tests::{
    IntegrationBoundary, IntegrationEnvironment, IntegrationGap, IntegrationSignal,
    IntegrationSurface, IntegrationTestDecision,
};

#[test]
fn consumer_can_describe_a_public_workflow_without_internal_access() {
    let decision = IntegrationTestDecision::new(
        "confirma un pago y actualiza el estado del pedido",
        IntegrationBoundary::PublicWorkflow,
        IntegrationSurface::PublicApi,
    )
    .expect("scenario is not empty")
    .with_gap(IntegrationGap::NonDeterministicInput);

    assert_eq!(
        decision.recommended_environment(),
        IntegrationEnvironment::InProcess
    );
    assert_eq!(decision.signal(), IntegrationSignal::Boundary);
}
