pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentBasis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<TimelineEventContentBasisBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TimelineEventContentBasisPriority>,
}

impl TimelineEventContentBasis {
    pub fn builder() -> TimelineEventContentBasisBuilder {
        <TimelineEventContentBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentBasisBuilder {
    basis: Option<TimelineEventContentBasisBasis>,
    priority: Option<TimelineEventContentBasisPriority>,
}

impl TimelineEventContentBasisBuilder {
    pub fn basis(mut self, value: TimelineEventContentBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: TimelineEventContentBasisPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentBasis`].
    pub fn build(self) -> Result<TimelineEventContentBasis, BuildError> {
        Ok(TimelineEventContentBasis {
            basis: self.basis,
            priority: self.priority,
        })
    }
}
