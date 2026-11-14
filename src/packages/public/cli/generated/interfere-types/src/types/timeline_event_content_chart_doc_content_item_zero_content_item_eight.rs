pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemEightAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemEightType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemEight {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemEightBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemEightBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemEightAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemEightType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemEight, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
