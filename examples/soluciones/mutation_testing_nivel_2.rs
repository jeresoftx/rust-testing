use rust_testing::mutation_testing::{
    MutationDecision, MutationGap, MutationKind, MutationOutcome,
};
fn main() -> Result<(), rust_testing::mutation_testing::MutationError> {
    let d = MutationDecision::new(
        "aplica límite de descuento",
        MutationKind::Boundary,
        MutationOutcome::Survived,
    )?
    .with_gap(MutationGap::CoverageOnly);
    println!("{} => señal: {:?}", d.rule(), d.signal());
    Ok(())
}
