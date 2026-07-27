use rust_testing::integration_tests::{
    IntegrationBoundary, IntegrationGap, IntegrationSurface, IntegrationTestDecision,
};

fn main() -> Result<(), rust_testing::integration_tests::IntegrationTestError> {
    let module_pair = IntegrationTestDecision::new(
        "normaliza una dirección y calcula el envío",
        IntegrationBoundary::ModulePair,
        IntegrationSurface::PublicApi,
    )?;

    let public_workflow = IntegrationTestDecision::new(
        "crea un pedido y reserva inventario",
        IntegrationBoundary::PublicWorkflow,
        IntegrationSurface::PublicApi,
    )?;

    let external_adapter = IntegrationTestDecision::new(
        "traduce la respuesta de un proveedor de pagos",
        IntegrationBoundary::ExternalAdapter,
        IntegrationSurface::ControlledEnvironment,
    )?
    .with_gap(IntegrationGap::UncontrolledInfrastructure);

    for decision in [module_pair, public_workflow, external_adapter] {
        println!(
            "- {} => entorno: {:?}, señal: {:?}, riesgos: {:?}",
            decision.scenario(),
            decision.recommended_environment(),
            decision.signal(),
            decision.gaps()
        );
    }

    Ok(())
}
