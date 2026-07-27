//! Modelo educativo para razonar sobre compatibilidad de contratos.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractDirection {
    ConsumerToProvider,
    ProviderToConsumer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    BackwardCompatible,
    CoordinatedChange,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractGap {
    MissingErrorCase,
    InternalDetail,
    UnversionedBreakingChange,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractSignal {
    Weak,
    Focused,
    Shared,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    EmptyOperation,
}

/// Contrato mínimo entre consumidor y proveedor.
///
/// ```
/// use rust_testing::contract_testing::{Compatibility, ContractDecision, ContractDirection};
/// let d = ContractDecision::new("consulta saldo", ContractDirection::ConsumerToProvider, Compatibility::BackwardCompatible)?;
/// assert_eq!(d.signal(), rust_testing::contract_testing::ContractSignal::Shared);
/// # Ok::<(), rust_testing::contract_testing::ContractError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDecision {
    operation: String,
    direction: ContractDirection,
    compatibility: Compatibility,
    gaps: Vec<ContractGap>,
}
impl ContractDecision {
    pub fn new(
        operation: impl Into<String>,
        direction: ContractDirection,
        compatibility: Compatibility,
    ) -> Result<Self, ContractError> {
        let operation = operation.into();
        if operation.trim().is_empty() {
            return Err(ContractError::EmptyOperation);
        }
        Ok(Self {
            operation,
            direction,
            compatibility,
            gaps: Vec::new(),
        })
    }
    pub fn operation(&self) -> &str {
        &self.operation
    }
    pub fn direction(&self) -> ContractDirection {
        self.direction
    }
    pub fn compatibility(&self) -> Compatibility {
        self.compatibility
    }
    pub fn with_gap(mut self, gap: ContractGap) -> Self {
        if !self.gaps.contains(&gap) {
            self.gaps.push(gap)
        }
        self
    }
    pub fn gaps(&self) -> &[ContractGap] {
        &self.gaps
    }
    pub fn signal(&self) -> ContractSignal {
        if self.gaps.contains(&ContractGap::UnversionedBreakingChange)
            || self.gaps.contains(&ContractGap::InternalDetail)
        {
            ContractSignal::Weak
        } else if self.gaps.contains(&ContractGap::MissingErrorCase) {
            ContractSignal::Focused
        } else {
            ContractSignal::Shared
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_empty() {
        assert_eq!(
            ContractDecision::new(
                " ",
                ContractDirection::ConsumerToProvider,
                Compatibility::BackwardCompatible
            ),
            Err(ContractError::EmptyOperation)
        );
    }
    #[test]
    fn shared_contract_is_strong() {
        let d = ContractDecision::new(
            "crea pedido",
            ContractDirection::ConsumerToProvider,
            Compatibility::BackwardCompatible,
        )
        .expect("not empty");
        assert_eq!(d.signal(), ContractSignal::Shared);
    }
    #[test]
    fn unversioned_change_is_weak() {
        let d = ContractDecision::new(
            "crea pedido",
            ContractDirection::ProviderToConsumer,
            Compatibility::CoordinatedChange,
        )
        .expect("not empty")
        .with_gap(ContractGap::UnversionedBreakingChange);
        assert_eq!(d.signal(), ContractSignal::Weak);
    }
}
