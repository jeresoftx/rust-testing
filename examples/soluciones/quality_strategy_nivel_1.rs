use rust_testing::quality_strategy::{QualityRisk, QualitySignal, StrategyDecision};
fn main() -> Result<(), rust_testing::quality_strategy::StrategyError> {
    let d = StrategyDecision::new(
        "protege precio",
        QualitySignal::LocalTest,
        QualityRisk::LocalRegression,
    )?;
    println!("{} => {:?}", d.decision(), d.signal());
    Ok(())
}
