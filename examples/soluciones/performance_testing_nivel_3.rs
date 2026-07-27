use rust_testing::performance_testing::{
    MeasurementDecision, MeasurementGap, MeasurementResult, MeasurementUnit,
};
fn main() -> Result<(), rust_testing::performance_testing::MeasurementError> {
    let d = MeasurementDecision::new(
        "crea factura",
        MeasurementUnit::Allocation,
        MeasurementResult::WithinBudget,
    )?
    .with_gap(MeasurementGap::ProductionClaim);
    println!("{} => {:?}", d.scenario(), d.signal());
    Ok(())
}
