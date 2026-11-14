pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFourAttrs {
    #[serde(rename = "surfaceChangeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_change_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFourAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemFourAttrsBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemFourAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFourAttrsBuilder {
    surface_change_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFourAttrsBuilder {
    pub fn surface_change_id(mut self, value: serde_json::Value) -> Self {
        self.surface_change_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemFourAttrs`].
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemFourAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemFourAttrs {
            surface_change_id: self.surface_change_id,
            snapshot: self.snapshot,
        })
    }
}
