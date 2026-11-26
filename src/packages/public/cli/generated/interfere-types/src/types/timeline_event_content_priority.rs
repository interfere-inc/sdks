pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentPriority {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<TimelineEventContentPriorityBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TimelineEventContentPriorityPriority>,
}

impl TimelineEventContentPriority {
    pub fn builder() -> TimelineEventContentPriorityBuilder {
        <TimelineEventContentPriorityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentPriorityBuilder {
    basis: Option<TimelineEventContentPriorityBasis>,
    priority: Option<TimelineEventContentPriorityPriority>,
}

impl TimelineEventContentPriorityBuilder {
    pub fn basis(mut self, value: TimelineEventContentPriorityBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: TimelineEventContentPriorityPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentPriority`].
    pub fn build(self) -> Result<TimelineEventContentPriority, BuildError> {
        Ok(TimelineEventContentPriority {
            basis: self.basis,
            priority: self.priority,
        })
    }
}
