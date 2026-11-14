pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemFive {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemFiveAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemFiveType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemFive {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemFiveType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemFive, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
