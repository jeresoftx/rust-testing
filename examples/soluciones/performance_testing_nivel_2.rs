use rust_testing::performance_testing::{
    MeasurementDecision, MeasurementGap, MeasurementResult, MeasurementUnit,
};
fn main() -> Result<(), rust_testing::performance_testing::MeasurementError> {
    let d = MeasurementDecision::new(
        "consulta catálogo",
        MeasurementUnit::Throughput,
        MeasurementResult::Inconclusive,
    )?
    .with_gap(MeasurementGap::MissingBaseline);
    println!("{} => {:?}", d.scenario(), d.signal());
    Ok(())
}
