pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractCompletionConcern {
    pub kind: TimelineEventRequestContractCompletionConcernKind,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    pub concern: TimelineEventRequestContractCompletionConcernConcern,
}

impl TimelineEventRequestContractCompletionConcern {
    pub fn builder() -> TimelineEventRequestContractCompletionConcernBuilder {
        <TimelineEventRequestContractCompletionConcernBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractCompletionConcernBuilder {
    kind: Option<TimelineEventRequestContractCompletionConcernKind>,
    target_id: Option<String>,
    concern: Option<TimelineEventRequestContractCompletionConcernConcern>,
}

impl TimelineEventRequestContractCompletionConcernBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractCompletionConcernKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn concern(mut self, value: TimelineEventRequestContractCompletionConcernConcern) -> Self {
        self.concern = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractCompletionConcern`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractCompletionConcernBuilder::kind)
    /// - [`concern`](TimelineEventRequestContractCompletionConcernBuilder::concern)
    pub fn build(self) -> Result<TimelineEventRequestContractCompletionConcern, BuildError> {
        Ok(TimelineEventRequestContractCompletionConcern {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            target_id: self.target_id,
            concern: self.concern.ok_or_else(|| BuildError::missing_field("concern"))?,
        })
    }
}
