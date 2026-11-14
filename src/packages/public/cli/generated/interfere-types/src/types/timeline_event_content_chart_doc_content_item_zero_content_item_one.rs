pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemOneAttrs,
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemOneType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemOne {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemOneBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemOneBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemOneAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemOneType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemOne, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
