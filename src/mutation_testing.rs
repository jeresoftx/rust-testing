//! Modelo educativo para interpretar el resultado de un mutante.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationKind {
    Condition,
    Boundary,
    ReturnValue,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationOutcome {
    Killed,
    Survived,
    Equivalent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationGap {
    MissingBehaviorAssertion,
    CoverageOnly,
    UninvestigatedEquivalent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MutationSignal {
    Weak,
    Investigate,
    Strong,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationError {
    EmptyRule,
}

/// Resultado de mutar una regla observable.
///
/// ```
/// use rust_testing::mutation_testing::{MutationDecision, MutationKind, MutationOutcome};
/// let d = MutationDecision::new("rechaza saldo negativo", MutationKind::Condition, MutationOutcome::Killed)?;
/// assert_eq!(d.signal(), rust_testing::mutation_testing::MutationSignal::Strong);
/// # Ok::<(), rust_testing::mutation_testing::MutationError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationDecision {
    rule: String,
    kind: MutationKind,
    outcome: MutationOutcome,
    gaps: Vec<MutationGap>,
}
impl MutationDecision {
    pub fn new(
        rule: impl Into<String>,
        kind: MutationKind,
        outcome: MutationOutcome,
    ) -> Result<Self, MutationError> {
        let rule = rule.into();
        if rule.trim().is_empty() {
            return Err(MutationError::EmptyRule);
        }
        Ok(Self {
            rule,
            kind,
            outcome,
            gaps: Vec::new(),
        })
    }
    pub fn rule(&self) -> &str {
        &self.rule
    }
    pub fn kind(&self) -> MutationKind {
        self.kind
    }
    pub fn outcome(&self) -> MutationOutcome {
        self.outcome
    }
    pub fn with_gap(mut self, gap: MutationGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap);
        }
        self
    }
    pub fn gaps(&self) -> &[MutationGap] {
        &self.gaps
    }
    pub fn signal(&self) -> MutationSignal {
        if self.gaps.contains(&MutationGap::CoverageOnly)
            || self.gaps.contains(&MutationGap::UninvestigatedEquivalent)
        {
            MutationSignal::Weak
        } else {
            match self.outcome {
                MutationOutcome::Killed => MutationSignal::Strong,
                MutationOutcome::Survived | MutationOutcome::Equivalent => {
                    MutationSignal::Investigate
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_empty() {
        assert_eq!(
            MutationDecision::new(" ", MutationKind::Condition, MutationOutcome::Killed),
            Err(MutationError::EmptyRule)
        );
    }
    #[test]
    fn killed_mutant_is_strong() {
        let d = MutationDecision::new(
            "rechaza saldo negativo",
            MutationKind::Condition,
            MutationOutcome::Killed,
        )
        .expect("not empty");
        assert_eq!(d.signal(), MutationSignal::Strong);
    }
    #[test]
    fn coverage_only_is_weak() {
        let d = MutationDecision::new(
            "limita descuento",
            MutationKind::Boundary,
            MutationOutcome::Survived,
        )
        .expect("not empty")
        .with_gap(MutationGap::CoverageOnly);
        assert_eq!(d.signal(), MutationSignal::Weak);
    }
}
