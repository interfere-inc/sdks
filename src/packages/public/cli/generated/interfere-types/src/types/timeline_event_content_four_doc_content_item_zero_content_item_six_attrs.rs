pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemSixAttrsBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: serde_json::Value) -> Self {
        self.observation_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemSixAttrs`].
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id,
            snapshot: self.snapshot,
        })
    }
}
