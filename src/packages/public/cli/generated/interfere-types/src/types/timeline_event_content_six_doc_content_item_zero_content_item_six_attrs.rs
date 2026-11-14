pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemSixAttrsBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: serde_json::Value) -> Self {
        self.observation_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemSixAttrs`].
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id,
            snapshot: self.snapshot,
        })
    }
}
