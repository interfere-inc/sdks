pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapHold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub kind: TimelineEventContentEvidenceGapHoldKind,
}

impl TimelineEventContentEvidenceGapHold {
    pub fn builder() -> TimelineEventContentEvidenceGapHoldBuilder {
        <TimelineEventContentEvidenceGapHoldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapHoldBuilder {
    category: Option<String>,
    kind: Option<TimelineEventContentEvidenceGapHoldKind>,
}

impl TimelineEventContentEvidenceGapHoldBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TimelineEventContentEvidenceGapHoldKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapHold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentEvidenceGapHoldBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapHold, BuildError> {
        Ok(TimelineEventContentEvidenceGapHold {
            category: self.category,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
