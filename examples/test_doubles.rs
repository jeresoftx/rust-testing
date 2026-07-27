use rust_testing::test_doubles::{DoubleContract, DoubleDecision, DoubleGap, DoubleKind};

fn main() -> Result<(), rust_testing::test_doubles::DoubleDecisionError> {
    let decisions = [
        DoubleDecision::new("reloj", DoubleKind::Stub, DoubleContract::Response)?,
        DoubleDecision::new(
            "repositorio en memoria",
            DoubleKind::Fake,
            DoubleContract::Behavior,
        )?,
        DoubleDecision::new(
            "publicador de eventos",
            DoubleKind::Mock,
            DoubleContract::Interaction,
        )?
        .with_gap(DoubleGap::ImplementationCoupling),
    ];
    for decision in decisions {
        println!(
            "{} => {:?}, señal: {:?}, riesgos: {:?}",
            decision.collaboration(),
            decision.kind(),
            decision.signal(),
            decision.gaps()
        );
    }
    Ok(())
}
