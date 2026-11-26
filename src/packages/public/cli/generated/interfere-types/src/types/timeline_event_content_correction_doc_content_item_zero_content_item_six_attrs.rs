pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrsBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: serde_json::Value) -> Self {
        self.observation_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs`].
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id,
            snapshot: self.snapshot,
        })
    }
}
