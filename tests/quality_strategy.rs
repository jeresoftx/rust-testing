use rust_testing::quality_strategy::{
    QualityRisk, QualitySignal, StrategyDecision, StrategySignal,
};
#[test]
fn consumer_can_model_operational_feedback() {
    let d = StrategyDecision::new(
        "observa errores reales",
        QualitySignal::Observability,
        QualityRisk::ProductionBehavior,
    )
    .expect("not empty");
    assert_eq!(d.signal(), StrategySignal::Complementary);
}
