use rust_testing::mutation_testing::{MutationDecision, MutationKind, MutationOutcome};
fn main() -> Result<(), rust_testing::mutation_testing::MutationError> {
    let d = MutationDecision::new(
        "rechaza descuento mayor al límite",
        MutationKind::Condition,
        MutationOutcome::Killed,
    )?;
    println!("{} => señal: {:?}", d.rule(), d.signal());
    Ok(())
}
