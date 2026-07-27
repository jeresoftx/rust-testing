//! Fundamentos de testing como modelo ejecutable.
//!
//! Este módulo no modela un framework. Modela una pregunta previa: qué tipo de
//! evidencia produce una prueba y qué huecos todavía conserva.

/// Tipo de evidencia que una prueba intenta producir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    /// Un ejemplo concreto de entrada y salida esperada.
    Example,
    /// Un caso límite que protege una frontera explícita.
    Boundary,
    /// Una propiedad que debe sostenerse para muchas entradas.
    Property,
    /// Un flujo que cruza módulos o componentes.
    Integration,
    /// Un contrato entre consumidor y proveedor.
    Contract,
    /// Una medición de comportamiento bajo costo, tiempo o carga.
    Performance,
}

impl EvidenceKind {
    fn baseline_signal(self) -> ConfidenceSignal {
        match self {
            Self::Example => ConfidenceSignal::Local,
            Self::Boundary | Self::Property | Self::Performance => ConfidenceSignal::Behavioral,
            Self::Integration | Self::Contract => ConfidenceSignal::Systemic,
        }
    }
}

/// Hueco conocido que reduce la confianza de una prueba.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceGap {
    /// La prueba ejecuta código, pero no afirma una expectativa clara.
    NoExplicitAssertion,
    /// La prueba puede fallar por tiempo, azar, red o estado externo.
    NonDeterministic,
    /// La prueba depende de un detalle interno en vez de comportamiento.
    ImplementationDetail,
    /// La prueba cubre el caso feliz, pero no la frontera relevante.
    MissingBoundary,
}

/// Señal de confianza producida por una afirmación de prueba.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfidenceSignal {
    /// Ejecuta código, pero aporta poca confianza.
    Cosmetic,
    /// Protege una regla local y concreta.
    Local,
    /// Protege comportamiento observable del dominio.
    Behavioral,
    /// Protege interacción entre componentes o contratos.
    Systemic,
}

impl ConfidenceSignal {
    fn downgraded(self) -> Self {
        match self {
            Self::Cosmetic | Self::Local => Self::Cosmetic,
            Self::Behavioral => Self::Local,
            Self::Systemic => Self::Behavioral,
        }
    }
}

/// Error al construir una afirmación de prueba.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimError {
    /// El comportamiento descrito está vacío o solo tiene espacios.
    EmptyBehavior,
}

/// Afirmación de prueba: comportamiento, evidencia y huecos conocidos.
///
/// ```
/// use rust_testing::fundamentals::{
///     ConfidenceSignal, EvidenceKind, TestClaim,
/// };
///
/// let claim = TestClaim::new(
///     "normaliza correos sin perder el dominio",
///     EvidenceKind::Boundary,
/// )?;
///
/// assert_eq!(claim.signal(), ConfidenceSignal::Behavioral);
/// # Ok::<(), rust_testing::fundamentals::ClaimError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestClaim {
    behavior: String,
    evidence: EvidenceKind,
    gaps: Vec<ConfidenceGap>,
}

impl TestClaim {
    /// Crea una afirmación de prueba con comportamiento observable.
    pub fn new(behavior: impl Into<String>, evidence: EvidenceKind) -> Result<Self, ClaimError> {
        let behavior = behavior.into();

        if behavior.trim().is_empty() {
            return Err(ClaimError::EmptyBehavior);
        }

        Ok(Self {
            behavior,
            evidence,
            gaps: Vec::new(),
        })
    }

    /// Describe el comportamiento que la prueba intenta proteger.
    pub fn behavior(&self) -> &str {
        &self.behavior
    }

    /// Devuelve el tipo de evidencia elegido.
    pub fn evidence(&self) -> EvidenceKind {
        self.evidence
    }

    /// Registra un hueco conocido de la afirmación.
    pub fn with_gap(mut self, gap: ConfidenceGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }
        self
    }

    /// Indica si la afirmación conserva un hueco específico.
    pub fn has_gap(&self, gap: ConfidenceGap) -> bool {
        self.gaps.contains(&gap)
    }

    /// Lista los huecos conocidos.
    pub fn gaps(&self) -> &[ConfidenceGap] {
        &self.gaps
    }

    /// Calcula la señal de confianza producida por la afirmación.
    ///
    /// Una prueba sin aserción explícita queda como señal cosmética aunque
    /// ejecute un flujo grande. Los huecos de determinismo o acoplamiento
    /// degradan la señal porque vuelven difícil interpretar una falla.
    pub fn signal(&self) -> ConfidenceSignal {
        if self.has_gap(ConfidenceGap::NoExplicitAssertion) {
            return ConfidenceSignal::Cosmetic;
        }

        let mut signal = self.evidence.baseline_signal();

        for gap in &self.gaps {
            match gap {
                ConfidenceGap::NoExplicitAssertion => {
                    return ConfidenceSignal::Cosmetic;
                }
                ConfidenceGap::NonDeterministic
                | ConfidenceGap::ImplementationDetail
                | ConfidenceGap::MissingBoundary => {
                    signal = signal.downgraded();
                }
            }
        }

        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_behavior() {
        let claim = TestClaim::new("   ", EvidenceKind::Example);

        assert_eq!(claim, Err(ClaimError::EmptyBehavior));
    }

    #[test]
    fn boundary_evidence_produces_behavioral_signal() {
        let claim = TestClaim::new(
            "rechaza una contraseña más corta que el mínimo",
            EvidenceKind::Boundary,
        )
        .expect("behavior is not empty");

        assert_eq!(claim.signal(), ConfidenceSignal::Behavioral);
    }

    #[test]
    fn missing_assertion_makes_signal_cosmetic() {
        let claim = TestClaim::new("recorre el flujo de checkout", EvidenceKind::Integration)
            .expect("behavior is not empty")
            .with_gap(ConfidenceGap::NoExplicitAssertion);

        assert_eq!(claim.signal(), ConfidenceSignal::Cosmetic);
    }

    #[test]
    fn implementation_detail_downgrades_signal() {
        let claim = TestClaim::new(
            "mantiene compatible el contrato de usuario público",
            EvidenceKind::Contract,
        )
        .expect("behavior is not empty")
        .with_gap(ConfidenceGap::ImplementationDetail);

        assert_eq!(claim.signal(), ConfidenceSignal::Behavioral);
    }

    #[test]
    fn duplicated_gap_is_stored_once() {
        let claim = TestClaim::new("ordena resultados por fecha", EvidenceKind::Example)
            .expect("behavior is not empty")
            .with_gap(ConfidenceGap::MissingBoundary)
            .with_gap(ConfidenceGap::MissingBoundary);

        assert_eq!(claim.gaps(), &[ConfidenceGap::MissingBoundary]);
    }
}
