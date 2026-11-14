pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs {
    #[serde(rename = "releaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<serde_json::Value>,
    #[serde(rename = "surfaceChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_change_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemFiveAttrsBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemFiveAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFiveAttrsBuilder {
    release_id: Option<serde_json::Value>,
    surface_change_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFiveAttrsBuilder {
    pub fn release_id(mut self, value: serde_json::Value) -> Self {
        self.release_id = Some(value);
        self
    }

    pub fn surface_change_id(mut self, value: serde_json::Value) -> Self {
        self.surface_change_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs`].
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs {
            release_id: self.release_id,
            surface_change_id: self.surface_change_id,
            snapshot: self.snapshot,
        })
    }
}
