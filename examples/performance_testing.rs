use rust_testing::performance_testing::{
    MeasurementDecision, MeasurementGap, MeasurementResult, MeasurementUnit,
};
fn main() -> Result<(), rust_testing::performance_testing::MeasurementError> {
    for d in [
        MeasurementDecision::new(
            "serializa pedido",
            MeasurementUnit::Latency,
            MeasurementResult::WithinBudget,
        )?,
        MeasurementDecision::new(
            "crea factura",
            MeasurementUnit::Allocation,
            MeasurementResult::Regression,
        )?,
        MeasurementDecision::new(
            "consulta catálogo",
            MeasurementUnit::Throughput,
            MeasurementResult::Inconclusive,
        )?
        .with_gap(MeasurementGap::MissingBaseline),
    ] {
        println!(
            "{} => {:?}, señal: {:?}",
            d.scenario(),
            d.result(),
            d.signal()
        );
    }
    Ok(())
}
