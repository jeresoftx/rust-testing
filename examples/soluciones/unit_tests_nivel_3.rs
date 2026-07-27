use rust_testing::unit_tests::{RuleVisibility, UnitBoundary, UnitTestDecision, UnitTestGap};

fn main() -> Result<(), rust_testing::unit_tests::UnitTestError> {
    let suite = [
        UnitTestDecision::new(
            "email vacío se rechaza dentro del módulo de validación",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )?,
        UnitTestDecision::new(
            "email sin arroba se rechaza dentro del módulo de validación",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )?,
        UnitTestDecision::new(
            "email válido produce un tipo normalizado",
            UnitBoundary::Type,
            RuleVisibility::Internal,
        )?
        .with_gap(UnitTestGap::MissingEdgeCase),
        UnitTestDecision::new(
            "el tipo público se puede construir desde texto",
            UnitBoundary::Type,
            RuleVisibility::PublicApi,
        )?,
    ];

    for decision in suite {
        println!(
            "{} => escala: {:?}, señal: {:?}, huecos: {:?}",
            decision.rule(),
            decision.recommended_scale(),
            decision.signal(),
            decision.gaps()
        );
    }

    Ok(())
}
