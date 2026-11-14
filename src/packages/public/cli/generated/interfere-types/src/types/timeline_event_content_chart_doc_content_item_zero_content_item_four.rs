pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemFourAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemFourType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemFour {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemFourBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemFourBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemFourAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemFourType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemFour, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
