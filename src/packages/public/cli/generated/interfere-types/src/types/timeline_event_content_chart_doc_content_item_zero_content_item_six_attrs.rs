pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemSixAttrsBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<serde_json::Value>,
    snapshot: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: serde_json::Value) -> Self {
        self.observation_id = Some(value);
        self
    }

    pub fn snapshot(mut self, value: serde_json::Value) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemSixAttrs`].
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id,
            snapshot: self.snapshot,
        })
    }
}
