use rust_testing::test_doubles::{DoubleContract, DoubleDecision, DoubleKind};

fn main() -> Result<(), rust_testing::test_doubles::DoubleDecisionError> {
    for decision in [
        DoubleDecision::new("hora fija", DoubleKind::Stub, DoubleContract::Response)?,
        DoubleDecision::new(
            "repositorio temporal",
            DoubleKind::Fake,
            DoubleContract::Behavior,
        )?,
        DoubleDecision::new(
            "evento de bienvenida",
            DoubleKind::Mock,
            DoubleContract::Interaction,
        )?,
    ] {
        println!(
            "{} => {:?}, señal: {:?}",
            decision.collaboration(),
            decision.kind(),
            decision.signal()
        );
    }
    Ok(())
}
