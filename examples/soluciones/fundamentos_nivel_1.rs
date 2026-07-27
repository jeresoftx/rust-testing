use rust_testing::fundamentals::{EvidenceKind, TestClaim};

fn main() -> Result<(), rust_testing::fundamentals::ClaimError> {
    let claims = [
        TestClaim::new(
            "rechaza una edad menor a 18 para una cuenta adulta",
            EvidenceKind::Boundary,
        )?,
        TestClaim::new(
            "serializar y deserializar conserva el mismo valor",
            EvidenceKind::Property,
        )?,
        TestClaim::new(
            "el cliente público no recibe campos internos del proveedor",
            EvidenceKind::Contract,
        )?,
        TestClaim::new(
            "el listado responde dentro del presupuesto esperado",
            EvidenceKind::Performance,
        )?,
    ];

    for claim in claims {
        println!(
            "{} => evidencia: {:?}, señal: {:?}",
            claim.behavior(),
            claim.evidence(),
            claim.signal()
        );
    }

    Ok(())
}
