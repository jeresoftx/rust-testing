use rust_testing::integration_tests::{
    IntegrationBoundary, IntegrationSurface, IntegrationTestDecision,
};

fn main() -> Result<(), rust_testing::integration_tests::IntegrationTestError> {
    let decisions = [
        IntegrationTestDecision::new(
            "normaliza una dirección y calcula el envío",
            IntegrationBoundary::ModulePair,
            IntegrationSurface::PublicApi,
        )?,
        IntegrationTestDecision::new(
            "crea un pedido y reserva inventario por una API pública",
            IntegrationBoundary::PublicWorkflow,
            IntegrationSurface::PublicApi,
        )?,
        IntegrationTestDecision::new(
            "traduce la respuesta de un proveedor de pagos con datos grabados",
            IntegrationBoundary::ExternalAdapter,
            IntegrationSurface::ContractFixture,
        )?,
    ];

    for decision in decisions {
        println!(
            "{} => frontera: {:?}, entorno: {:?}",
            decision.scenario(),
            decision.boundary(),
            decision.recommended_environment()
        );
    }

    Ok(())
}
