use rust_testing::fundamentals::{ConfidenceGap, EvidenceKind, TestClaim};

fn main() -> Result<(), rust_testing::fundamentals::ClaimError> {
    let suite = [
        TestClaim::new(
            "credenciales válidas crean una sesión",
            EvidenceKind::Example,
        )?,
        TestClaim::new(
            "contraseña corta se rechaza antes de autenticar",
            EvidenceKind::Boundary,
        )?,
        TestClaim::new(
            "cliente y API sostienen el contrato público de sesión",
            EvidenceKind::Contract,
        )?,
        TestClaim::new(
            "inicio de sesión completo funciona con proveedor externo",
            EvidenceKind::Integration,
        )?
        .with_gap(ConfidenceGap::NonDeterministic),
    ];

    for claim in suite {
        println!(
            "{} => evidencia: {:?}, señal: {:?}, huecos: {:?}",
            claim.behavior(),
            claim.evidence(),
            claim.signal(),
            claim.gaps()
        );
    }

    Ok(())
}
