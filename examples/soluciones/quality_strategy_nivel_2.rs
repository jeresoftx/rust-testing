use rust_testing::quality_strategy::{QualityRisk, QualitySignal, StrategyDecision, StrategyGap};
fn main() -> Result<(), rust_testing::quality_strategy::StrategyError> {
    let d = StrategyDecision::new(
        "aprueba cambio",
        QualitySignal::ContinuousIntegration,
        QualityRisk::KnownRegression,
    )?
    .with_gap(StrategyGap::AutomationAsApproval);
    println!("{} => {:?}", d.decision(), d.signal());
    Ok(())
}
