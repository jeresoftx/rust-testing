//! Modelo educativo para decidir el alcance de una prueba de integración.
//!
//! El módulo representa la decisión anterior al framework: qué contrato cruza
//! una frontera, cómo se observa desde fuera y qué riesgos reducen la señal.

/// Frontera que el escenario integrado necesita cruzar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationBoundary {
    /// Dos módulos colaboran dentro del mismo crate.
    ModulePair,
    /// Una API pública coordina varios tipos o componentes.
    PublicWorkflow,
    /// Un adaptador conecta el dominio con una dependencia externa controlada.
    ExternalAdapter,
}

/// Superficie desde la que el consumidor observa el contrato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationSurface {
    /// El escenario usa la API pública del crate en el mismo proceso.
    PublicApi,
    /// El escenario usa un contrato explícito con datos de prueba controlados.
    ContractFixture,
    /// El escenario requiere un entorno aislado para una dependencia externa.
    ControlledEnvironment,
}

/// Riesgo que hace menos confiable una prueba de integración.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationGap {
    /// La prueba comparte datos o estado con otros escenarios.
    SharedState,
    /// El escenario depende del reloj real o de valores no deterministas.
    NonDeterministicInput,
    /// La prueba usa red o infraestructura fuera de un entorno controlado.
    UncontrolledInfrastructure,
    /// El caso solo verifica el flujo feliz de una colaboración.
    MissingFailurePath,
}

/// Estrategia de entorno recomendada para obtener evidencia útil.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationEnvironment {
    /// Ejecutar el flujo desde la API pública, sin infraestructura externa.
    InProcess,
    /// Usar datos o respuestas controladas para un contrato externo.
    Fixture,
    /// Aislar una dependencia externa en un entorno reproducible.
    Sandbox,
}

/// Calidad esperada de la evidencia de integración.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegrationSignal {
    /// El escenario ejecuta código, pero su resultado depende demasiado del entorno.
    Weak,
    /// El escenario protege una colaboración delimitada.
    Boundary,
    /// El escenario protege un flujo público completo y reproducible.
    Flow,
}

impl IntegrationSignal {
    fn downgraded(self) -> Self {
        match self {
            Self::Weak | Self::Boundary => Self::Weak,
            Self::Flow => Self::Boundary,
        }
    }
}

/// Error al construir una decisión de integración.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationTestError {
    /// El escenario descrito está vacío o solo contiene espacios.
    EmptyScenario,
}

/// Decisión sobre una prueba de integración.
///
/// ```
/// use rust_testing::integration_tests::{
///     IntegrationBoundary, IntegrationEnvironment, IntegrationSurface,
///     IntegrationTestDecision,
/// };
///
/// let decision = IntegrationTestDecision::new(
///     "crea un pedido y reserva inventario por la API pública",
///     IntegrationBoundary::PublicWorkflow,
///     IntegrationSurface::PublicApi,
/// )?;
///
/// assert_eq!(decision.recommended_environment(), IntegrationEnvironment::InProcess);
/// # Ok::<(), rust_testing::integration_tests::IntegrationTestError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationTestDecision {
    scenario: String,
    boundary: IntegrationBoundary,
    surface: IntegrationSurface,
    gaps: Vec<IntegrationGap>,
}

impl IntegrationTestDecision {
    /// Crea una decisión para un flujo que cruza una frontera de diseño.
    pub fn new(
        scenario: impl Into<String>,
        boundary: IntegrationBoundary,
        surface: IntegrationSurface,
    ) -> Result<Self, IntegrationTestError> {
        let scenario = scenario.into();

        if scenario.trim().is_empty() {
            return Err(IntegrationTestError::EmptyScenario);
        }

        Ok(Self {
            scenario,
            boundary,
            surface,
            gaps: Vec::new(),
        })
    }

    /// Flujo observable que el consumidor debe poder completar.
    pub fn scenario(&self) -> &str {
        &self.scenario
    }

    /// Frontera de colaboración que cubre el escenario.
    pub fn boundary(&self) -> IntegrationBoundary {
        self.boundary
    }

    /// Superficie desde la que se observa el contrato.
    pub fn surface(&self) -> IntegrationSurface {
        self.surface
    }

    /// Registra un riesgo conocido sin duplicarlo.
    pub fn with_gap(mut self, gap: IntegrationGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }

        self
    }

    /// Riesgos conocidos del escenario.
    pub fn gaps(&self) -> &[IntegrationGap] {
        &self.gaps
    }

    /// Recomienda el entorno mínimo que conserva el contrato observable.
    pub fn recommended_environment(&self) -> IntegrationEnvironment {
        match self.surface {
            IntegrationSurface::PublicApi => IntegrationEnvironment::InProcess,
            IntegrationSurface::ContractFixture => IntegrationEnvironment::Fixture,
            IntegrationSurface::ControlledEnvironment => IntegrationEnvironment::Sandbox,
        }
    }

    /// Calcula la señal esperada de la decisión.
    pub fn signal(&self) -> IntegrationSignal {
        if self
            .gaps
            .contains(&IntegrationGap::UncontrolledInfrastructure)
            || self.gaps.contains(&IntegrationGap::SharedState)
        {
            return IntegrationSignal::Weak;
        }

        let mut signal = match self.boundary {
            IntegrationBoundary::ModulePair | IntegrationBoundary::ExternalAdapter => {
                IntegrationSignal::Boundary
            }
            IntegrationBoundary::PublicWorkflow => IntegrationSignal::Flow,
        };

        for gap in &self.gaps {
            if matches!(
                gap,
                IntegrationGap::NonDeterministicInput | IntegrationGap::MissingFailurePath
            ) {
                signal = signal.downgraded();
            }
        }

        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_scenario() {
        let decision = IntegrationTestDecision::new(
            "   ",
            IntegrationBoundary::ModulePair,
            IntegrationSurface::PublicApi,
        );

        assert_eq!(decision, Err(IntegrationTestError::EmptyScenario));
    }

    #[test]
    fn public_workflow_uses_in_process_environment_with_flow_signal() {
        let decision = IntegrationTestDecision::new(
            "crea un pedido y reserva inventario",
            IntegrationBoundary::PublicWorkflow,
            IntegrationSurface::PublicApi,
        )
        .expect("scenario is not empty");

        assert_eq!(
            decision.recommended_environment(),
            IntegrationEnvironment::InProcess
        );
        assert_eq!(decision.signal(), IntegrationSignal::Flow);
    }

    #[test]
    fn contract_fixture_uses_fixture_environment() {
        let decision = IntegrationTestDecision::new(
            "traduce la respuesta de un proveedor de pagos",
            IntegrationBoundary::ExternalAdapter,
            IntegrationSurface::ContractFixture,
        )
        .expect("scenario is not empty");

        assert_eq!(
            decision.recommended_environment(),
            IntegrationEnvironment::Fixture
        );
        assert_eq!(decision.signal(), IntegrationSignal::Boundary);
    }

    #[test]
    fn uncontrolled_infrastructure_makes_signal_weak() {
        let decision = IntegrationTestDecision::new(
            "sincroniza una factura con un servicio externo",
            IntegrationBoundary::ExternalAdapter,
            IntegrationSurface::ControlledEnvironment,
        )
        .expect("scenario is not empty")
        .with_gap(IntegrationGap::UncontrolledInfrastructure);

        assert_eq!(decision.signal(), IntegrationSignal::Weak);
    }

    #[test]
    fn repeated_gap_is_recorded_once_and_downgrades_flow() {
        let decision = IntegrationTestDecision::new(
            "registra usuario y entrega correo de bienvenida",
            IntegrationBoundary::PublicWorkflow,
            IntegrationSurface::PublicApi,
        )
        .expect("scenario is not empty")
        .with_gap(IntegrationGap::MissingFailurePath)
        .with_gap(IntegrationGap::MissingFailurePath);

        assert_eq!(decision.signal(), IntegrationSignal::Boundary);
        assert_eq!(decision.gaps(), &[IntegrationGap::MissingFailurePath]);
    }
}
