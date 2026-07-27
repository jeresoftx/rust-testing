//! Modelo educativo para decidir cuándo una prueba debe ser un unit test.
//!
//! El módulo no reemplaza `#[test]`. Representa la decisión previa: qué unidad
//! se protege, qué tan visible es la regla y qué escala de evidencia conviene.

/// Frontera que la prueba intenta proteger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitBoundary {
    /// Una función pura o casi pura con entradas y salidas claras.
    Function,
    /// Un método o tipo con invariantes propios.
    Type,
    /// Un módulo que conserva detalles internos y expone una API pequeña.
    Module,
}

/// Visibilidad de la regla que se desea probar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleVisibility {
    /// La regla forma parte de la API pública y debe enseñarse como ejemplo.
    PublicApi,
    /// La regla vive dentro del módulo y no debe hacerse pública solo para testear.
    Internal,
}

/// Hueco de diseño que reduce el valor de una prueba unitaria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitTestGap {
    /// La prueba está amarrada a un detalle accidental de implementación.
    ImplementationCoupling,
    /// La prueba cubre solo el caso feliz y deja fuera el borde relevante.
    MissingEdgeCase,
    /// La regla necesita interacción entre módulos para producir evidencia útil.
    CrossesModuleBoundary,
}

/// Escala de prueba recomendada para una regla.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestScale {
    /// `#[test]` cerca del código, normalmente dentro de `#[cfg(test)]`.
    Unit,
    /// Doctest para comportamiento público que debe enseñarse desde la API.
    Doctest,
    /// Test de integración cuando la evidencia necesita cruzar módulos.
    Integration,
}

/// Señal esperada de una decisión de unit testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitSignal {
    /// La prueba ejecuta código, pero dice poco sobre diseño.
    Weak,
    /// La prueba protege una regla pequeña y local.
    Local,
    /// La prueba protege una frontera del módulo.
    Boundary,
    /// La prueba documenta comportamiento público con valor educativo.
    PublicContract,
}

impl UnitSignal {
    fn downgraded(self) -> Self {
        match self {
            Self::Weak | Self::Local => Self::Weak,
            Self::Boundary => Self::Local,
            Self::PublicContract => Self::Boundary,
        }
    }
}

/// Error al construir una decisión de prueba unitaria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitTestError {
    /// La regla descrita está vacía o solo contiene espacios.
    EmptyRule,
}

/// Decisión sobre una prueba unitaria.
///
/// ```
/// use rust_testing::unit_tests::{
///     RuleVisibility, TestScale, UnitBoundary, UnitTestDecision, UnitSignal,
/// };
///
/// let decision = UnitTestDecision::new(
///     "rechaza emails sin arroba antes de persistir",
///     UnitBoundary::Module,
///     RuleVisibility::Internal,
/// )?;
///
/// assert_eq!(decision.recommended_scale(), TestScale::Unit);
/// assert_eq!(decision.signal(), UnitSignal::Boundary);
/// # Ok::<(), rust_testing::unit_tests::UnitTestError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitTestDecision {
    rule: String,
    boundary: UnitBoundary,
    visibility: RuleVisibility,
    gaps: Vec<UnitTestGap>,
}

impl UnitTestDecision {
    /// Crea una decisión para una regla pequeña de diseño.
    pub fn new(
        rule: impl Into<String>,
        boundary: UnitBoundary,
        visibility: RuleVisibility,
    ) -> Result<Self, UnitTestError> {
        let rule = rule.into();

        if rule.trim().is_empty() {
            return Err(UnitTestError::EmptyRule);
        }

        Ok(Self {
            rule,
            boundary,
            visibility,
            gaps: Vec::new(),
        })
    }

    /// Regla observable que se desea proteger.
    pub fn rule(&self) -> &str {
        &self.rule
    }

    /// Frontera de diseño asociada a la regla.
    pub fn boundary(&self) -> UnitBoundary {
        self.boundary
    }

    /// Visibilidad de la regla.
    pub fn visibility(&self) -> RuleVisibility {
        self.visibility
    }

    /// Registra un hueco conocido sin duplicarlo.
    pub fn with_gap(mut self, gap: UnitTestGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }

        self
    }

    /// Huecos conocidos de la decisión.
    pub fn gaps(&self) -> &[UnitTestGap] {
        &self.gaps
    }

    /// Recomienda la escala de prueba más honesta para la regla.
    pub fn recommended_scale(&self) -> TestScale {
        if self.gaps.contains(&UnitTestGap::CrossesModuleBoundary) {
            return TestScale::Integration;
        }

        match self.visibility {
            RuleVisibility::PublicApi => TestScale::Doctest,
            RuleVisibility::Internal => TestScale::Unit,
        }
    }

    /// Calcula la señal esperada de la decisión.
    pub fn signal(&self) -> UnitSignal {
        let mut signal = match (self.boundary, self.visibility) {
            (_, RuleVisibility::PublicApi) => UnitSignal::PublicContract,
            (UnitBoundary::Module, RuleVisibility::Internal) => UnitSignal::Boundary,
            (UnitBoundary::Type, RuleVisibility::Internal)
            | (UnitBoundary::Function, RuleVisibility::Internal) => UnitSignal::Local,
        };

        for gap in &self.gaps {
            match gap {
                UnitTestGap::CrossesModuleBoundary => return UnitSignal::Weak,
                UnitTestGap::ImplementationCoupling | UnitTestGap::MissingEdgeCase => {
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
    fn rejects_empty_rule() {
        let decision =
            UnitTestDecision::new("   ", UnitBoundary::Function, RuleVisibility::Internal);

        assert_eq!(decision, Err(UnitTestError::EmptyRule));
    }

    #[test]
    fn internal_module_rule_prefers_unit_test_with_boundary_signal() {
        let decision = UnitTestDecision::new(
            "normaliza el correo antes de comparar dominios",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )
        .expect("rule is not empty");

        assert_eq!(decision.recommended_scale(), TestScale::Unit);
        assert_eq!(decision.signal(), UnitSignal::Boundary);
    }

    #[test]
    fn public_api_rule_prefers_doctest() {
        let decision = UnitTestDecision::new(
            "parsea una ruta pública desde texto",
            UnitBoundary::Type,
            RuleVisibility::PublicApi,
        )
        .expect("rule is not empty");

        assert_eq!(decision.recommended_scale(), TestScale::Doctest);
        assert_eq!(decision.signal(), UnitSignal::PublicContract);
    }

    #[test]
    fn crossing_modules_moves_recommendation_to_integration() {
        let decision = UnitTestDecision::new(
            "sincroniza carrito y catálogo",
            UnitBoundary::Module,
            RuleVisibility::Internal,
        )
        .expect("rule is not empty")
        .with_gap(UnitTestGap::CrossesModuleBoundary);

        assert_eq!(decision.recommended_scale(), TestScale::Integration);
        assert_eq!(decision.signal(), UnitSignal::Weak);
    }

    #[test]
    fn implementation_coupling_downgrades_signal_once() {
        let decision = UnitTestDecision::new(
            "mantiene estable el orden público de errores",
            UnitBoundary::Type,
            RuleVisibility::PublicApi,
        )
        .expect("rule is not empty")
        .with_gap(UnitTestGap::ImplementationCoupling)
        .with_gap(UnitTestGap::ImplementationCoupling);

        assert_eq!(decision.signal(), UnitSignal::Boundary);
        assert_eq!(decision.gaps(), &[UnitTestGap::ImplementationCoupling]);
    }
}
