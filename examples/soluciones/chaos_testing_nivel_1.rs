use rust_testing::chaos_testing::{BlastRadius, ChaosDecision, ExperimentOutcome, FailureKind};
fn main() -> Result<(), rust_testing::chaos_testing::ChaosError> {
    let d = ChaosDecision::new(
        "reintenta solicitud",
        FailureKind::Latency,
        BlastRadius::Local,
        ExperimentOutcome::Recovered,
    )?;
    println!("{} => {:?}", d.hypothesis(), d.signal());
    Ok(())
}
