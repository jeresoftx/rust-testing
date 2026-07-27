use rust_testing::integration_tests::{
    IntegrationBoundary, IntegrationGap, IntegrationSurface, IntegrationTestDecision,
};

fn main() -> Result<(), rust_testing::integration_tests::IntegrationTestError> {
    let reproducible_flow = IntegrationTestDecision::new(
        "registra usuario y prepara bienvenida desde la API pública",
        IntegrationBoundary::PublicWorkflow,
        IntegrationSurface::PublicApi,
    )?;

    let fragile_flow = IntegrationTestDecision::new(
        "rechaza correo duplicado sin crear un segundo usuario",
        IntegrationBoundary::PublicWorkflow,
        IntegrationSurface::PublicApi,
    )?
    .with_gap(IntegrationGap::SharedState);

    for decision in [reproducible_flow, fragile_flow] {
        println!(
            "{} => entorno: {:?}, señal: {:?}, riesgos: {:?}",
            decision.scenario(),
            decision.recommended_environment(),
            decision.signal(),
            decision.gaps()
        );
    }

    Ok(())
}
