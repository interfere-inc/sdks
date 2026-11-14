pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentChartDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentChartDocContentItemZeroType,
}

impl TimelineEventContentChartDocContentItemZero {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroBuilder {
        <TimelineEventContentChartDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentChartDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentChartDocContentItemZeroType>,
}

impl TimelineEventContentChartDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentChartDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZero, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
