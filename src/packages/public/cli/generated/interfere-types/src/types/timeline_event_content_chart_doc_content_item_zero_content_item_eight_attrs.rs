pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemEightAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemEightAttrs {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemEightAttrsBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemEightAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemEightAttrsBuilder {
    label: Option<serde_json::Value>,
    value: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemEightAttrsBuilder {
    pub fn label(mut self, value: serde_json::Value) -> Self {
        self.label = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemEightAttrs`].
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemEightAttrs, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemEightAttrs {
            label: self.label,
            value: self.value,
        })
    }
}
