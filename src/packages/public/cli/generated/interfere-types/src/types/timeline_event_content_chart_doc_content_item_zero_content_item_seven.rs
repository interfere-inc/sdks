pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemSevenAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemSevenType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSeven {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemSevenType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemSeven, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
