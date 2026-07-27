//! Modelo educativo para diseñar experimentos de falla controlados.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Latency,
    DependencyUnavailable,
    InvalidResponse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlastRadius {
    Local,
    IsolatedFlow,
    SharedEnvironment,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentOutcome {
    Recovered,
    Degraded,
    Unexpected,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChaosGap {
    MissingStopCondition,
    UnboundedScope,
    MissingHypothesis,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChaosSignal {
    Unsafe,
    Investigate,
    Controlled,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChaosError {
    EmptyHypothesis,
}

/// Decisión mínima para un experimento de falla controlado.
///
/// ```
/// use rust_testing::chaos_testing::{BlastRadius, ChaosDecision, ExperimentOutcome, FailureKind};
/// let d = ChaosDecision::new("el flujo conserva la solicitud", FailureKind::Latency, BlastRadius::Local, ExperimentOutcome::Recovered)?;
/// assert_eq!(d.signal(), rust_testing::chaos_testing::ChaosSignal::Controlled);
/// # Ok::<(), rust_testing::chaos_testing::ChaosError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChaosDecision {
    hypothesis: String,
    failure: FailureKind,
    radius: BlastRadius,
    outcome: ExperimentOutcome,
    gaps: Vec<ChaosGap>,
}
impl ChaosDecision {
    pub fn new(
        hypothesis: impl Into<String>,
        failure: FailureKind,
        radius: BlastRadius,
        outcome: ExperimentOutcome,
    ) -> Result<Self, ChaosError> {
        let hypothesis = hypothesis.into();
        if hypothesis.trim().is_empty() {
            return Err(ChaosError::EmptyHypothesis);
        }
        Ok(Self {
            hypothesis,
            failure,
            radius,
            outcome,
            gaps: Vec::new(),
        })
    }
    pub fn hypothesis(&self) -> &str {
        &self.hypothesis
    }
    pub fn failure(&self) -> FailureKind {
        self.failure
    }
    pub fn radius(&self) -> BlastRadius {
        self.radius
    }
    pub fn outcome(&self) -> ExperimentOutcome {
        self.outcome
    }
    pub fn with_gap(mut self, gap: ChaosGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap)
        }
        self
    }
    pub fn gaps(&self) -> &[ChaosGap] {
        &self.gaps
    }
    pub fn signal(&self) -> ChaosSignal {
        if self.radius == BlastRadius::SharedEnvironment
            || self.gaps.contains(&ChaosGap::UnboundedScope)
            || self.gaps.contains(&ChaosGap::MissingStopCondition)
        {
            ChaosSignal::Unsafe
        } else {
            match self.outcome {
                ExperimentOutcome::Recovered => ChaosSignal::Controlled,
                ExperimentOutcome::Degraded | ExperimentOutcome::Unexpected => {
                    ChaosSignal::Investigate
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
            ChaosDecision::new(
                " ",
                FailureKind::Latency,
                BlastRadius::Local,
                ExperimentOutcome::Recovered
            ),
            Err(ChaosError::EmptyHypothesis)
        );
    }
    #[test]
    fn local_recovery_is_controlled() {
        let d = ChaosDecision::new(
            "reintenta",
            FailureKind::Latency,
            BlastRadius::Local,
            ExperimentOutcome::Recovered,
        )
        .expect("not empty");
        assert_eq!(d.signal(), ChaosSignal::Controlled);
    }
    #[test]
    fn unbounded_scope_is_unsafe() {
        let d = ChaosDecision::new(
            "recupera",
            FailureKind::DependencyUnavailable,
            BlastRadius::IsolatedFlow,
            ExperimentOutcome::Degraded,
        )
        .expect("not empty")
        .with_gap(ChaosGap::UnboundedScope);
        assert_eq!(d.signal(), ChaosSignal::Unsafe);
    }
}
