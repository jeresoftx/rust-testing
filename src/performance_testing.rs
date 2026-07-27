//! Modelo educativo para interpretar una medición de rendimiento.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementUnit {
    Latency,
    Throughput,
    Allocation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementResult {
    WithinBudget,
    Regression,
    Inconclusive,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementGap {
    MissingBaseline,
    NoisyEnvironment,
    ProductionClaim,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeasurementSignal {
    Weak,
    Investigate,
    Useful,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementError {
    EmptyScenario,
}

/// Decisión mínima sobre una medición de rendimiento.
///
/// ```
/// use rust_testing::performance_testing::{MeasurementDecision, MeasurementResult, MeasurementUnit};
/// let d = MeasurementDecision::new("serializa pedido", MeasurementUnit::Latency, MeasurementResult::WithinBudget)?;
/// assert_eq!(d.signal(), rust_testing::performance_testing::MeasurementSignal::Useful);
/// # Ok::<(), rust_testing::performance_testing::MeasurementError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementDecision {
    scenario: String,
    unit: MeasurementUnit,
    result: MeasurementResult,
    gaps: Vec<MeasurementGap>,
}
impl MeasurementDecision {
    pub fn new(
        scenario: impl Into<String>,
        unit: MeasurementUnit,
        result: MeasurementResult,
    ) -> Result<Self, MeasurementError> {
        let scenario = scenario.into();
        if scenario.trim().is_empty() {
            return Err(MeasurementError::EmptyScenario);
        }
        Ok(Self {
            scenario,
            unit,
            result,
            gaps: Vec::new(),
        })
    }
    pub fn scenario(&self) -> &str {
        &self.scenario
    }
    pub fn unit(&self) -> MeasurementUnit {
        self.unit
    }
    pub fn result(&self) -> MeasurementResult {
        self.result
    }
    pub fn with_gap(mut self, gap: MeasurementGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap)
        }
        self
    }
    pub fn gaps(&self) -> &[MeasurementGap] {
        &self.gaps
    }
    pub fn signal(&self) -> MeasurementSignal {
        if self.gaps.contains(&MeasurementGap::MissingBaseline)
            || self.gaps.contains(&MeasurementGap::ProductionClaim)
        {
            MeasurementSignal::Weak
        } else {
            match self.result {
                MeasurementResult::WithinBudget => MeasurementSignal::Useful,
                MeasurementResult::Regression | MeasurementResult::Inconclusive => {
                    MeasurementSignal::Investigate
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
            MeasurementDecision::new(
                " ",
                MeasurementUnit::Latency,
                MeasurementResult::WithinBudget
            ),
            Err(MeasurementError::EmptyScenario)
        );
    }
    #[test]
    fn within_budget_is_useful() {
        let d = MeasurementDecision::new(
            "serializa pedido",
            MeasurementUnit::Latency,
            MeasurementResult::WithinBudget,
        )
        .expect("not empty");
        assert_eq!(d.signal(), MeasurementSignal::Useful);
    }
    #[test]
    fn missing_baseline_is_weak() {
        let d = MeasurementDecision::new(
            "consulta catálogo",
            MeasurementUnit::Throughput,
            MeasurementResult::Regression,
        )
        .expect("not empty")
        .with_gap(MeasurementGap::MissingBaseline);
        assert_eq!(d.signal(), MeasurementSignal::Weak);
    }
}
