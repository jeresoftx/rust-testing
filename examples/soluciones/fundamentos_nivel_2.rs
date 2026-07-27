use rust_testing::fundamentals::{ConfidenceGap, EvidenceKind, TestClaim};

fn main() -> Result<(), rust_testing::fundamentals::ClaimError> {
    let claims = [
        TestClaim::new("recorre el checkout completo", EvidenceKind::Integration)?
            .with_gap(ConfidenceGap::NoExplicitAssertion),
        TestClaim::new(
            "genera un token válido antes de expirar",
            EvidenceKind::Boundary,
        )?
        .with_gap(ConfidenceGap::NonDeterministic),
        TestClaim::new(
            "mantiene compatible el contrato público de usuario",
            EvidenceKind::Contract,
        )?
        .with_gap(ConfidenceGap::ImplementationDetail),
        TestClaim::new(
            "acepta contraseñas con longitud suficiente",
            EvidenceKind::Example,
        )?
        .with_gap(ConfidenceGap::MissingBoundary),
    ];

    for claim in claims {
        println!(
            "{} => huecos: {:?}, señal: {:?}",
            claim.behavior(),
            claim.gaps(),
            claim.signal()
        );
    }

    Ok(())
}
