pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentChartDocContentItem>,
    pub r#type: TimelineEventContentChartDocType,
}

impl TimelineEventContentChartDoc {
    pub fn builder() -> TimelineEventContentChartDocBuilder {
        <TimelineEventContentChartDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocBuilder {
    content: Option<Vec<TimelineEventContentChartDocContentItem>>,
    r#type: Option<TimelineEventContentChartDocType>,
}

impl TimelineEventContentChartDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentChartDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentChartDocBuilder::content)
    /// - [`r#type`](TimelineEventContentChartDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDoc, BuildError> {
        Ok(TimelineEventContentChartDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
