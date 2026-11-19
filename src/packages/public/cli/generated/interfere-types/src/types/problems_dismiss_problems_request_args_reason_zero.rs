pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DismissProblemsRequestArgsReasonZero {
    pub kind: DismissProblemsRequestArgsReasonZeroKind,
}

impl DismissProblemsRequestArgsReasonZero {
    pub fn builder() -> DismissProblemsRequestArgsReasonZeroBuilder {
        <DismissProblemsRequestArgsReasonZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DismissProblemsRequestArgsReasonZeroBuilder {
    kind: Option<DismissProblemsRequestArgsReasonZeroKind>,
}

impl DismissProblemsRequestArgsReasonZeroBuilder {
    pub fn kind(mut self, value: DismissProblemsRequestArgsReasonZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DismissProblemsRequestArgsReasonZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](DismissProblemsRequestArgsReasonZeroBuilder::kind)
    pub fn build(self) -> Result<DismissProblemsRequestArgsReasonZero, BuildError> {
        Ok(DismissProblemsRequestArgsReasonZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
