pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentChartDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentChartDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentChartDocContentItemOneType,
}

impl TimelineEventContentChartDocContentItemOne {
    pub fn builder() -> TimelineEventContentChartDocContentItemOneBuilder {
        <TimelineEventContentChartDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentChartDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentChartDocContentItemOneType>,
}

impl TimelineEventContentChartDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentChartDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentChartDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemOne, BuildError> {
        Ok(TimelineEventContentChartDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
