use rust_testing::property_testing::{GeneratorDomain, PropertyDecision, PropertyKind};
fn main() -> Result<(), rust_testing::property_testing::PropertyError> {
    for d in [
        PropertyDecision::new(
            "normalizar dos veces conserva resultado",
            PropertyKind::Idempotence,
            GeneratorDomain::ValidInputs,
        )?,
        PropertyDecision::new(
            "serializar y leer conserva valor",
            PropertyKind::RoundTrip,
            GeneratorDomain::EdgeFocused,
        )?,
        PropertyDecision::new(
            "ordenar conserva longitud",
            PropertyKind::Invariant,
            GeneratorDomain::BoundedValues,
        )?,
    ] {
        println!("{} => {:?}", d.statement(), d.kind());
    }
    Ok(())
}
