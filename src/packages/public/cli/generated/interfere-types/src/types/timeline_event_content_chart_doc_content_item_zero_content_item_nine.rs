pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemNine {
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemZeroContentItemNineAttrs,
}

impl TimelineEventContentChartDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemNineBuilder {
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemNineType>,
    attrs: Option<TimelineEventContentChartDocContentItemZeroContentItemNineAttrs>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TimelineEventContentChartDocContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
