use rust_testing::quality_strategy::{QualityRisk, QualitySignal, StrategyDecision, StrategyGap};
fn main() -> Result<(), rust_testing::quality_strategy::StrategyError> {
    for d in [
        StrategyDecision::new(
            "protege invariantes de precio",
            QualitySignal::LocalTest,
            QualityRisk::LocalRegression,
        )?,
        StrategyDecision::new(
            "observa errores reales",
            QualitySignal::Observability,
            QualityRisk::ProductionBehavior,
        )?,
        StrategyDecision::new(
            "aprueba cambio",
            QualitySignal::ContinuousIntegration,
            QualityRisk::KnownRegression,
        )?
        .with_gap(StrategyGap::AutomationAsApproval),
    ] {
        println!("{} => {:?}", d.decision(), d.signal());
    }
    Ok(())
}
