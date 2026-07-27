//! Modelo educativo para elegir un doble de prueba con intención explícita.

/// Clase de doble de prueba.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleKind {
    /// Devuelve una respuesta preparada para un escenario conocido.
    Stub,
    /// Implementación funcional, simple y controlada de una colaboración.
    Fake,
    /// Verifica una interacción que forma parte del contrato observable.
    Mock,
}

/// Contrato que el doble debe conservar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleContract {
    /// Solo importa una respuesta determinista para el escenario.
    Response,
    /// Importan reglas simples de colaboración, como almacenar o recuperar datos.
    Behavior,
    /// Importa que una interacción observable ocurra con cierta intención.
    Interaction,
}

/// Riesgo que vuelve engañoso el uso de un doble.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoubleGap {
    /// El doble verifica una coreografía interna accidental.
    ImplementationCoupling,
    /// El fake implementa reglas diferentes a las que debe representar.
    DivergentDomainRule,
    /// El escenario usa más simulación de la necesaria.
    OverSimulation,
}

/// Señal esperada de una decisión sobre dobles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoubleSignal {
    /// El doble permite ejecutar la prueba, pero dice poco sobre el contrato.
    Weak,
    /// El doble conserva una parte delimitada del contrato.
    Focused,
    /// El doble preserva la evidencia principal sin exceso de simulación.
    Contractual,
}

impl DoubleSignal {
    fn downgraded(self) -> Self {
        match self {
            Self::Weak | Self::Focused => Self::Weak,
            Self::Contractual => Self::Focused,
        }
    }
}

/// Error al construir una decisión sobre un doble.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoubleDecisionError {
    /// La colaboración descrita está vacía o contiene solo espacios.
    EmptyCollaboration,
}

/// Decisión sobre el doble que una prueba necesita.
///
/// ```
/// use rust_testing::test_doubles::{DoubleContract, DoubleKind, DoubleDecision};
///
/// let decision = DoubleDecision::new(
///     "proveedor de tipo de cambio",
///     DoubleKind::Stub,
///     DoubleContract::Response,
/// )?;
///
/// assert_eq!(decision.signal(), rust_testing::test_doubles::DoubleSignal::Contractual);
/// # Ok::<(), rust_testing::test_doubles::DoubleDecisionError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleDecision {
    collaboration: String,
    kind: DoubleKind,
    contract: DoubleContract,
    gaps: Vec<DoubleGap>,
}

impl DoubleDecision {
    /// Crea una decisión para una colaboración sustituida.
    pub fn new(
        collaboration: impl Into<String>,
        kind: DoubleKind,
        contract: DoubleContract,
    ) -> Result<Self, DoubleDecisionError> {
        let collaboration = collaboration.into();
        if collaboration.trim().is_empty() {
            return Err(DoubleDecisionError::EmptyCollaboration);
        }
        Ok(Self {
            collaboration,
            kind,
            contract,
            gaps: Vec::new(),
        })
    }

    /// Colaboración sustituida por la prueba.
    pub fn collaboration(&self) -> &str {
        &self.collaboration
    }

    /// Clase de doble elegida.
    pub fn kind(&self) -> DoubleKind {
        self.kind
    }

    /// Contrato que el doble conserva.
    pub fn contract(&self) -> DoubleContract {
        self.contract
    }

    /// Registra un riesgo conocido sin duplicarlo.
    pub fn with_gap(mut self, gap: DoubleGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }
        self
    }

    /// Riesgos conocidos de la decisión.
    pub fn gaps(&self) -> &[DoubleGap] {
        &self.gaps
    }

    /// Calcula la señal de acuerdo con la correspondencia entre doble y contrato.
    pub fn signal(&self) -> DoubleSignal {
        if self.gaps.contains(&DoubleGap::ImplementationCoupling)
            || self.gaps.contains(&DoubleGap::DivergentDomainRule)
        {
            return DoubleSignal::Weak;
        }

        let mut signal = match (self.kind, self.contract) {
            (DoubleKind::Stub, DoubleContract::Response)
            | (DoubleKind::Fake, DoubleContract::Behavior)
            | (DoubleKind::Mock, DoubleContract::Interaction) => DoubleSignal::Contractual,
            _ => DoubleSignal::Focused,
        };
        if self.gaps.contains(&DoubleGap::OverSimulation) {
            signal = signal.downgraded();
        }
        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_collaboration() {
        assert_eq!(
            DoubleDecision::new(" ", DoubleKind::Stub, DoubleContract::Response),
            Err(DoubleDecisionError::EmptyCollaboration)
        );
    }

    #[test]
    fn matching_stub_preserves_response_contract() {
        let decision = DoubleDecision::new("reloj", DoubleKind::Stub, DoubleContract::Response)
            .expect("collaboration is not empty");
        assert_eq!(decision.signal(), DoubleSignal::Contractual);
    }

    #[test]
    fn fake_with_divergent_rules_is_weak() {
        let decision =
            DoubleDecision::new("repositorio", DoubleKind::Fake, DoubleContract::Behavior)
                .expect("collaboration is not empty")
                .with_gap(DoubleGap::DivergentDomainRule);
        assert_eq!(decision.signal(), DoubleSignal::Weak);
    }

    #[test]
    fn over_simulation_downgrades_matching_mock_once() {
        let decision =
            DoubleDecision::new("notificador", DoubleKind::Mock, DoubleContract::Interaction)
                .expect("collaboration is not empty")
                .with_gap(DoubleGap::OverSimulation)
                .with_gap(DoubleGap::OverSimulation);
        assert_eq!(decision.signal(), DoubleSignal::Focused);
        assert_eq!(decision.gaps(), &[DoubleGap::OverSimulation]);
    }
}
