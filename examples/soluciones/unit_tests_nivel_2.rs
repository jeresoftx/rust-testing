use rust_testing::unit_tests::{RuleVisibility, UnitBoundary, UnitTestDecision, UnitTestGap};

fn main() -> Result<(), rust_testing::unit_tests::UnitTestError> {
    let decisions = [
        UnitTestDecision::new(
            "mantiene el orden interno de pasos privados",
            UnitBoundary::Function,
            RuleVisibility::Internal,
        )?
        .with_gap(UnitTestGap::ImplementationCoupling),
        UnitTestDecision::new(
            "acepta edades dentro del rango permitido",
            UnitBoundary::Function,
            RuleVisibility::Internal,
        )?
        .with_gap(UnitTestGap::MissingEdgeCase),
        UnitTestDecision::new(
            "valida usuario contra base de datos y API",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )?
        .with_gap(UnitTestGap::CrossesModuleBoundary),
    ];

    for decision in decisions {
        println!(
            "{} => escala: {:?}, huecos: {:?}, señal: {:?}",
            decision.rule(),
            decision.recommended_scale(),
            decision.gaps(),
            decision.signal()
        );
    }

    Ok(())
}
