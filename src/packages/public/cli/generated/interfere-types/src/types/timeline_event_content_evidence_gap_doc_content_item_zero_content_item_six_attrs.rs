pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: serde_json::Value) -> Self {
        self.observation_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id,
            snapshot: self.snapshot,
        })
    }
}
