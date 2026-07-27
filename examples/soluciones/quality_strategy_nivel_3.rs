use rust_testing::quality_strategy::{QualityRisk, QualitySignal, StrategyDecision};
fn main() -> Result<(), rust_testing::quality_strategy::StrategyError> {
    let d = StrategyDecision::new(
        "observa error real",
        QualitySignal::Observability,
        QualityRisk::ProductionBehavior,
    )?;
    println!("{} => {:?}", d.decision(), d.signal());
    Ok(())
}
