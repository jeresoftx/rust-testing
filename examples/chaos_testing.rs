use rust_testing::chaos_testing::{
    BlastRadius, ChaosDecision, ChaosGap, ExperimentOutcome, FailureKind,
};
fn main() -> Result<(), rust_testing::chaos_testing::ChaosError> {
    for d in [
        ChaosDecision::new(
            "reintenta solicitud",
            FailureKind::Latency,
            BlastRadius::Local,
            ExperimentOutcome::Recovered,
        )?,
        ChaosDecision::new(
            "muestra degradación",
            FailureKind::InvalidResponse,
            BlastRadius::Local,
            ExperimentOutcome::Degraded,
        )?,
        ChaosDecision::new(
            "recupera dependencia",
            FailureKind::DependencyUnavailable,
            BlastRadius::IsolatedFlow,
            ExperimentOutcome::Recovered,
        )?
        .with_gap(ChaosGap::MissingStopCondition),
    ] {
        println!("{} => {:?}", d.hypothesis(), d.signal());
    }
    Ok(())
}
