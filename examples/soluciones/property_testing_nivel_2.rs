use rust_testing::property_testing::{
    GeneratorDomain, PropertyDecision, PropertyGap, PropertyKind,
};
fn main() -> Result<(), rust_testing::property_testing::PropertyError> {
    let d = PropertyDecision::new(
        "no falla",
        PropertyKind::Invariant,
        GeneratorDomain::BoundedValues,
    )?
    .with_gap(PropertyGap::VagueClaim);
    println!("{} => {:?}", d.statement(), d.signal());
    Ok(())
}
