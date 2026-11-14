pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemTwoAttrsBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<serde_json::Value>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs`].
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs {
            name: self.name,
        })
    }
}
