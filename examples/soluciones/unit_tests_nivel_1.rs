use rust_testing::unit_tests::{RuleVisibility, UnitBoundary, UnitTestDecision};

fn main() -> Result<(), rust_testing::unit_tests::UnitTestError> {
    let decisions = [
        UnitTestDecision::new(
            "normaliza un correo antes de comparar dominios",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )?,
        UnitTestDecision::new(
            "parsea una ruta pública desde texto",
            UnitBoundary::Type,
            RuleVisibility::PublicApi,
        )?,
        UnitTestDecision::new(
            "rechaza un precio negativo antes de guardar",
            UnitBoundary::Function,
            RuleVisibility::Internal,
        )?,
    ];

    for decision in decisions {
        println!(
            "{} => escala: {:?}, señal: {:?}",
            decision.rule(),
            decision.recommended_scale(),
            decision.signal()
        );
    }

    Ok(())
}
