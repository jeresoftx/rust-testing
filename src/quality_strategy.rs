//! Modelo educativo para asignar señales de calidad a riesgos concretos.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualitySignal {
    LocalTest,
    IntegrationTest,
    ContinuousIntegration,
    Observability,
    HumanReview,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityRisk {
    LocalRegression,
    BoundaryMismatch,
    KnownRegression,
    ProductionBehavior,
    IntentOrTradeoff,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyGap {
    DuplicateSignal,
    MissingOperationalFeedback,
    AutomationAsApproval,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrategySignal {
    Weak,
    Focused,
    Complementary,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategyError {
    EmptyDecision,
}

/// Decisión sobre una señal y el riesgo que pretende reducir.
///
/// ```
/// use rust_testing::quality_strategy::{QualityRisk, QualitySignal, StrategyDecision};
/// let d = StrategyDecision::new("protege invariantes de precio", QualitySignal::LocalTest, QualityRisk::LocalRegression)?;
/// assert_eq!(d.signal(), rust_testing::quality_strategy::StrategySignal::Complementary);
/// # Ok::<(), rust_testing::quality_strategy::StrategyError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyDecision {
    decision: String,
    source: QualitySignal,
    risk: QualityRisk,
    gaps: Vec<StrategyGap>,
}
impl StrategyDecision {
    pub fn new(
        decision: impl Into<String>,
        source: QualitySignal,
        risk: QualityRisk,
    ) -> Result<Self, StrategyError> {
        let decision = decision.into();
        if decision.trim().is_empty() {
            return Err(StrategyError::EmptyDecision);
        }
        Ok(Self {
            decision,
            source,
            risk,
            gaps: Vec::new(),
        })
    }
    pub fn decision(&self) -> &str {
        &self.decision
    }
    pub fn source(&self) -> QualitySignal {
        self.source
    }
    pub fn risk(&self) -> QualityRisk {
        self.risk
    }
    pub fn with_gap(mut self, gap: StrategyGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap)
        }
        self
    }
    pub fn gaps(&self) -> &[StrategyGap] {
        &self.gaps
    }
    pub fn signal(&self) -> StrategySignal {
        if self.gaps.contains(&StrategyGap::AutomationAsApproval)
            || self.gaps.contains(&StrategyGap::DuplicateSignal)
        {
            StrategySignal::Weak
        } else if self.gaps.contains(&StrategyGap::MissingOperationalFeedback) {
            StrategySignal::Focused
        } else {
            StrategySignal::Complementary
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_empty() {
        assert_eq!(
            StrategyDecision::new(" ", QualitySignal::LocalTest, QualityRisk::LocalRegression),
            Err(StrategyError::EmptyDecision)
        );
    }
    #[test]
    fn clear_signal_is_complementary() {
        let d = StrategyDecision::new(
            "protege precio",
            QualitySignal::LocalTest,
            QualityRisk::LocalRegression,
        )
        .expect("not empty");
        assert_eq!(d.signal(), StrategySignal::Complementary);
    }
    #[test]
    fn approval_automation_is_weak() {
        let d = StrategyDecision::new(
            "aprueba cambio",
            QualitySignal::ContinuousIntegration,
            QualityRisk::KnownRegression,
        )
        .expect("not empty")
        .with_gap(StrategyGap::AutomationAsApproval);
        assert_eq!(d.signal(), StrategySignal::Weak);
    }
}
