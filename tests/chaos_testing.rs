use rust_testing::chaos_testing::{
    BlastRadius, ChaosDecision, ChaosSignal, ExperimentOutcome, FailureKind,
};
#[test]
fn consumer_can_investigate_degradation() {
    let d = ChaosDecision::new(
        "muestra degradación",
        FailureKind::InvalidResponse,
        BlastRadius::Local,
        ExperimentOutcome::Degraded,
    )
    .expect("not empty");
    assert_eq!(d.signal(), ChaosSignal::Investigate);
}
