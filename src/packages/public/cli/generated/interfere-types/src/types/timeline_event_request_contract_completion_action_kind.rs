pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractCompletionActionKind {
    pub kind: TimelineEventRequestContractCompletionActionKindKind,
    #[serde(rename = "actionKind")]
    pub action_kind: TimelineEventRequestContractCompletionActionKindActionKind,
}

impl TimelineEventRequestContractCompletionActionKind {
    pub fn builder() -> TimelineEventRequestContractCompletionActionKindBuilder {
        <TimelineEventRequestContractCompletionActionKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractCompletionActionKindBuilder {
    kind: Option<TimelineEventRequestContractCompletionActionKindKind>,
    action_kind: Option<TimelineEventRequestContractCompletionActionKindActionKind>,
}

impl TimelineEventRequestContractCompletionActionKindBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractCompletionActionKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn action_kind(mut self, value: TimelineEventRequestContractCompletionActionKindActionKind) -> Self {
        self.action_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractCompletionActionKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractCompletionActionKindBuilder::kind)
    /// - [`action_kind`](TimelineEventRequestContractCompletionActionKindBuilder::action_kind)
    pub fn build(self) -> Result<TimelineEventRequestContractCompletionActionKind, BuildError> {
        Ok(TimelineEventRequestContractCompletionActionKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            action_kind: self.action_kind.ok_or_else(|| BuildError::missing_field("action_kind"))?,
        })
    }
}
