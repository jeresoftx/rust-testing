use rust_testing::unit_tests::{RuleVisibility, UnitBoundary, UnitTestDecision, UnitTestGap};

fn main() -> Result<(), rust_testing::unit_tests::UnitTestError> {
    let internal_rule = UnitTestDecision::new(
        "normaliza el correo antes de comparar dominios",
        UnitBoundary::Module,
        RuleVisibility::Internal,
    )?;

    let public_rule = UnitTestDecision::new(
        "parsea una ruta pública desde texto",
        UnitBoundary::Type,
        RuleVisibility::PublicApi,
    )?;

    let cross_module_rule = UnitTestDecision::new(
        "sincroniza carrito y catálogo",
        UnitBoundary::Module,
        RuleVisibility::Internal,
    )?
    .with_gap(UnitTestGap::CrossesModuleBoundary);

    for decision in [internal_rule, public_rule, cross_module_rule] {
        println!(
            "- {} => escala: {:?}, señal: {:?}, huecos: {:?}",
            decision.rule(),
            decision.recommended_scale(),
            decision.signal(),
            decision.gaps()
        );
    }

    Ok(())
}
