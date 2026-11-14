pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemNineAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemNineAttrsBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemNineAttrsBuilder {
    value: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemNineAttrs`].
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemNineAttrs {
            value: self.value,
        })
    }
}
