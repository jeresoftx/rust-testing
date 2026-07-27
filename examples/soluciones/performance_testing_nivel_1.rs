use rust_testing::performance_testing::{MeasurementDecision, MeasurementResult, MeasurementUnit};
fn main() -> Result<(), rust_testing::performance_testing::MeasurementError> {
    let d = MeasurementDecision::new(
        "serializa pedido",
        MeasurementUnit::Latency,
        MeasurementResult::WithinBudget,
    )?;
    println!("{} => {:?}", d.scenario(), d.signal());
    Ok(())
}
