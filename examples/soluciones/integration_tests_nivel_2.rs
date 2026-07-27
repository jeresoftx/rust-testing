use rust_testing::integration_tests::{
    IntegrationBoundary, IntegrationGap, IntegrationSurface, IntegrationTestDecision,
};

fn main() -> Result<(), rust_testing::integration_tests::IntegrationTestError> {
    let decisions = [
        IntegrationTestDecision::new(
            "dos escenarios reutilizan el mismo usuario persistido",
            IntegrationBoundary::PublicWorkflow,
            IntegrationSurface::PublicApi,
        )?
        .with_gap(IntegrationGap::SharedState),
        IntegrationTestDecision::new(
            "el vencimiento de una factura depende de la hora real",
            IntegrationBoundary::ModulePair,
            IntegrationSurface::PublicApi,
        )?
        .with_gap(IntegrationGap::NonDeterministicInput),
        IntegrationTestDecision::new(
            "un pedido se confirma sin verificar falta de stock",
            IntegrationBoundary::PublicWorkflow,
            IntegrationSurface::PublicApi,
        )?
        .with_gap(IntegrationGap::MissingFailurePath),
    ];

    for decision in decisions {
        println!(
            "{} => riesgos: {:?}, señal: {:?}",
            decision.scenario(),
            decision.gaps(),
            decision.signal()
        );
    }

    Ok(())
}
