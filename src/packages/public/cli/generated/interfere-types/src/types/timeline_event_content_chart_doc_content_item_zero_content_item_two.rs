pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemTwoType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemTwo {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemTwoType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemTwo, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
