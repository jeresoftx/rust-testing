//! Modelo educativo para formular propiedades antes de elegir un generador.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    Idempotence,
    RoundTrip,
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorDomain {
    BoundedValues,
    ValidInputs,
    EdgeFocused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyGap {
    VagueClaim,
    MissingEdge,
    NonReproducibleCounterexample,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertySignal {
    Weak,
    Focused,
    Generative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyError {
    EmptyStatement,
}

/// Declaración mínima de una propiedad verificable.
///
/// ```
/// use rust_testing::property_testing::{GeneratorDomain, PropertyDecision, PropertyKind};
/// let decision = PropertyDecision::new("normalizar dos veces no cambia el resultado", PropertyKind::Idempotence, GeneratorDomain::ValidInputs)?;
/// assert_eq!(decision.signal(), rust_testing::property_testing::PropertySignal::Generative);
/// # Ok::<(), rust_testing::property_testing::PropertyError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDecision {
    statement: String,
    kind: PropertyKind,
    domain: GeneratorDomain,
    gaps: Vec<PropertyGap>,
}

impl PropertyDecision {
    pub fn new(
        statement: impl Into<String>,
        kind: PropertyKind,
        domain: GeneratorDomain,
    ) -> Result<Self, PropertyError> {
        let statement = statement.into();
        if statement.trim().is_empty() {
            return Err(PropertyError::EmptyStatement);
        }
        Ok(Self {
            statement,
            kind,
            domain,
            gaps: Vec::new(),
        })
    }
    pub fn statement(&self) -> &str {
        &self.statement
    }
    pub fn kind(&self) -> PropertyKind {
        self.kind
    }
    pub fn domain(&self) -> GeneratorDomain {
        self.domain
    }
    pub fn with_gap(mut self, gap: PropertyGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }
        self
    }
    pub fn gaps(&self) -> &[PropertyGap] {
        &self.gaps
    }
    pub fn signal(&self) -> PropertySignal {
        if self.gaps.contains(&PropertyGap::VagueClaim)
            || self
                .gaps
                .contains(&PropertyGap::NonReproducibleCounterexample)
        {
            return PropertySignal::Weak;
        }
        if self.gaps.contains(&PropertyGap::MissingEdge) {
            return PropertySignal::Focused;
        }
        PropertySignal::Generative
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_empty_property() {
        assert_eq!(
            PropertyDecision::new(" ", PropertyKind::Invariant, GeneratorDomain::BoundedValues),
            Err(PropertyError::EmptyStatement)
        );
    }
    #[test]
    fn explicit_property_has_generative_signal() {
        let d = PropertyDecision::new(
            "serializar y leer conserva el valor",
            PropertyKind::RoundTrip,
            GeneratorDomain::ValidInputs,
        )
        .expect("not empty");
        assert_eq!(d.signal(), PropertySignal::Generative);
    }
    #[test]
    fn vague_claim_is_weak() {
        let d = PropertyDecision::new(
            "no falla",
            PropertyKind::Invariant,
            GeneratorDomain::BoundedValues,
        )
        .expect("not empty")
        .with_gap(PropertyGap::VagueClaim);
        assert_eq!(d.signal(), PropertySignal::Weak);
    }
}
