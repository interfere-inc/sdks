pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractCompletionGapKind {
    pub kind: TimelineEventRequestContractCompletionGapKindKind,
    #[serde(rename = "gapKind")]
    pub gap_kind: TimelineEventRequestContractCompletionGapKindGapKind,
}

impl TimelineEventRequestContractCompletionGapKind {
    pub fn builder() -> TimelineEventRequestContractCompletionGapKindBuilder {
        <TimelineEventRequestContractCompletionGapKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractCompletionGapKindBuilder {
    kind: Option<TimelineEventRequestContractCompletionGapKindKind>,
    gap_kind: Option<TimelineEventRequestContractCompletionGapKindGapKind>,
}

impl TimelineEventRequestContractCompletionGapKindBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractCompletionGapKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn gap_kind(mut self, value: TimelineEventRequestContractCompletionGapKindGapKind) -> Self {
        self.gap_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractCompletionGapKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractCompletionGapKindBuilder::kind)
    /// - [`gap_kind`](TimelineEventRequestContractCompletionGapKindBuilder::gap_kind)
    pub fn build(self) -> Result<TimelineEventRequestContractCompletionGapKind, BuildError> {
        Ok(TimelineEventRequestContractCompletionGapKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            gap_kind: self.gap_kind.ok_or_else(|| BuildError::missing_field("gap_kind"))?,
        })
    }
}
