use rust_testing::performance_testing::{
    MeasurementDecision, MeasurementResult, MeasurementSignal, MeasurementUnit,
};
#[test]
fn consumer_can_request_investigation() {
    let d = MeasurementDecision::new(
        "crea factura",
        MeasurementUnit::Allocation,
        MeasurementResult::Regression,
    )
    .expect("not empty");
    assert_eq!(d.signal(), MeasurementSignal::Investigate);
}
