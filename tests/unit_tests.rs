use rust_testing::unit_tests::{
    RuleVisibility, TestScale, UnitBoundary, UnitSignal, UnitTestDecision, UnitTestGap,
};

#[test]
fn missing_edge_case_keeps_unit_scale_but_reduces_signal() {
    let decision = UnitTestDecision::new(
        "acepta edades dentro del rango permitido",
        UnitBoundary::Function,
        RuleVisibility::Internal,
    )
    .expect("rule is not empty")
    .with_gap(UnitTestGap::MissingEdgeCase);

    assert_eq!(decision.recommended_scale(), TestScale::Unit);
    assert_eq!(decision.signal(), UnitSignal::Weak);
}

#[test]
fn public_contract_can_stay_documented_as_doctest() {
    let decision = UnitTestDecision::new(
        "formatea errores públicos para clientes HTTP",
        UnitBoundary::Type,
        RuleVisibility::PublicApi,
    )
    .expect("rule is not empty");

    assert_eq!(decision.recommended_scale(), TestScale::Doctest);
    assert_eq!(decision.signal(), UnitSignal::PublicContract);
    assert!(decision.gaps().is_empty());
}
