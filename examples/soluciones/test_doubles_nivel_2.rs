use rust_testing::test_doubles::{DoubleContract, DoubleDecision, DoubleGap, DoubleKind};

fn main() -> Result<(), rust_testing::test_doubles::DoubleDecisionError> {
    for decision in [
        DoubleDecision::new(
            "fake de repositorio",
            DoubleKind::Fake,
            DoubleContract::Behavior,
        )?
        .with_gap(DoubleGap::DivergentDomainRule),
        DoubleDecision::new(
            "mock de notificador",
            DoubleKind::Mock,
            DoubleContract::Interaction,
        )?
        .with_gap(DoubleGap::ImplementationCoupling),
    ] {
        println!(
            "{} => riesgos: {:?}, señal: {:?}",
            decision.collaboration(),
            decision.gaps(),
            decision.signal()
        );
    }
    Ok(())
}
