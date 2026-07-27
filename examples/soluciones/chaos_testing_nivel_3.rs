use rust_testing::chaos_testing::{BlastRadius, ChaosDecision, ExperimentOutcome, FailureKind};
fn main() -> Result<(), rust_testing::chaos_testing::ChaosError> {
    let d = ChaosDecision::new(
        "muestra degradación controlada",
        FailureKind::InvalidResponse,
        BlastRadius::Local,
        ExperimentOutcome::Degraded,
    )?;
    println!("{} => {:?}", d.hypothesis(), d.signal());
    Ok(())
}
