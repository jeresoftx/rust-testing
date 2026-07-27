use rust_testing::property_testing::{GeneratorDomain, PropertyDecision, PropertyKind};
fn main() -> Result<(), rust_testing::property_testing::PropertyError> {
    let d = PropertyDecision::new(
        "guardar y leer preferencia conserva valor",
        PropertyKind::RoundTrip,
        GeneratorDomain::EdgeFocused,
    )?;
    println!("{} => {:?}", d.statement(), d.signal());
    Ok(())
}
