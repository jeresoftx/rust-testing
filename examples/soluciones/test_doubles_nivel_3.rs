use rust_testing::test_doubles::{DoubleContract, DoubleDecision, DoubleKind};

fn main() -> Result<(), rust_testing::test_doubles::DoubleDecisionError> {
    let fake = DoubleDecision::new(
        "usuarios temporales",
        DoubleKind::Fake,
        DoubleContract::Behavior,
    )?;
    let mock = DoubleDecision::new(
        "evento de bienvenida",
        DoubleKind::Mock,
        DoubleContract::Interaction,
    )?;
    println!("{} => señal: {:?}", fake.collaboration(), fake.signal());
    println!("{} => señal: {:?}", mock.collaboration(), mock.signal());
    Ok(())
}
