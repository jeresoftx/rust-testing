use rust_testing::fundamentals::{ConfidenceGap, EvidenceKind, TestClaim};

fn main() -> Result<(), rust_testing::fundamentals::ClaimError> {
    let happy_path = TestClaim::new(
        "crea una sesión cuando las credenciales son válidas",
        EvidenceKind::Example,
    )?;

    let boundary = TestClaim::new(
        "rechaza una contraseña más corta que el mínimo permitido",
        EvidenceKind::Boundary,
    )?;

    let unstable_integration = TestClaim::new(
        "sincroniza inventario entre catálogo y carrito",
        EvidenceKind::Integration,
    )?
    .with_gap(ConfidenceGap::NonDeterministic);

    for claim in [happy_path, boundary, unstable_integration] {
        println!(
            "- {} => evidencia: {:?}, señal: {:?}, huecos: {:?}",
            claim.behavior(),
            claim.evidence(),
            claim.signal(),
            claim.gaps()
        );
    }

    Ok(())
}
