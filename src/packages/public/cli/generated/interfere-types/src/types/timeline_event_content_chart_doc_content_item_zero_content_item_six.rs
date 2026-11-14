pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemSixAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemSixType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSix {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemSixBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemSixBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemSixAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemSixType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemSix, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
