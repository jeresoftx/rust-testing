use rust_testing::fundamentals::{ConfidenceGap, ConfidenceSignal, EvidenceKind, TestClaim};

#[test]
fn property_claim_with_clear_assertion_is_behavioral_evidence() {
    let claim = TestClaim::new(
        "la serialización seguida de deserialización conserva el valor",
        EvidenceKind::Property,
    )
    .expect("behavior is not empty");

    assert_eq!(claim.signal(), ConfidenceSignal::Behavioral);
    assert_eq!(claim.evidence(), EvidenceKind::Property);
}

#[test]
fn non_deterministic_integration_test_loses_systemic_strength() {
    let claim = TestClaim::new(
        "sincroniza inventario entre catálogo y carrito",
        EvidenceKind::Integration,
    )
    .expect("behavior is not empty")
    .with_gap(ConfidenceGap::NonDeterministic);

    assert_eq!(claim.signal(), ConfidenceSignal::Behavioral);
    assert!(claim.has_gap(ConfidenceGap::NonDeterministic));
}
