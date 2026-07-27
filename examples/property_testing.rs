use rust_testing::property_testing::{
    GeneratorDomain, PropertyDecision, PropertyGap, PropertyKind,
};
fn main() -> Result<(), rust_testing::property_testing::PropertyError> {
    for decision in [
        PropertyDecision::new(
            "normalizar dos veces no cambia el resultado",
            PropertyKind::Idempotence,
            GeneratorDomain::ValidInputs,
        )?,
        PropertyDecision::new(
            "serializar y leer conserva el valor",
            PropertyKind::RoundTrip,
            GeneratorDomain::EdgeFocused,
        )?,
        PropertyDecision::new(
            "no falla",
            PropertyKind::Invariant,
            GeneratorDomain::BoundedValues,
        )?
        .with_gap(PropertyGap::VagueClaim),
    ] {
        println!(
            "{} => {:?}, señal: {:?}",
            decision.statement(),
            decision.kind(),
            decision.signal()
        );
    }
    Ok(())
}
