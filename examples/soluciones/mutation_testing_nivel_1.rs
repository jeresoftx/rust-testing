use rust_testing::mutation_testing::{MutationDecision, MutationKind, MutationOutcome};
fn main() -> Result<(), rust_testing::mutation_testing::MutationError> {
    for d in [
        MutationDecision::new(
            "rechaza saldo negativo",
            MutationKind::Condition,
            MutationOutcome::Killed,
        )?,
        MutationDecision::new(
            "limita descuento",
            MutationKind::Boundary,
            MutationOutcome::Survived,
        )?,
        MutationDecision::new(
            "devuelve total",
            MutationKind::ReturnValue,
            MutationOutcome::Equivalent,
        )?,
    ] {
        println!("{} => {:?}, señal: {:?}", d.rule(), d.outcome(), d.signal());
    }
    Ok(())
}
