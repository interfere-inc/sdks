pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs {
    #[serde(rename = "surfaceChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_change_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrsBuilder {
    surface_change_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrsBuilder {
    pub fn surface_change_id(mut self, value: serde_json::Value) -> Self {
        self.surface_change_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs {
            surface_change_id: self.surface_change_id,
            snapshot: self.snapshot,
        })
    }
}
