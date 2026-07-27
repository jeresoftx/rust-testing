use rust_testing::chaos_testing::{
    BlastRadius, ChaosDecision, ChaosGap, ExperimentOutcome, FailureKind,
};
fn main() -> Result<(), rust_testing::chaos_testing::ChaosError> {
    let d = ChaosDecision::new(
        "recupera dependencia",
        FailureKind::DependencyUnavailable,
        BlastRadius::IsolatedFlow,
        ExperimentOutcome::Recovered,
    )?
    .with_gap(ChaosGap::MissingStopCondition);
    println!("{} => {:?}", d.hypothesis(), d.signal());
    Ok(())
}
