pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveOneDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentTwelveOneDocContentItem>,
    pub r#type: TimelineEventContentTwelveOneDocType,
}

impl TimelineEventContentTwelveOneDoc {
    pub fn builder() -> TimelineEventContentTwelveOneDocBuilder {
        <TimelineEventContentTwelveOneDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocBuilder {
    content: Option<Vec<TimelineEventContentTwelveOneDocContentItem>>,
    r#type: Option<TimelineEventContentTwelveOneDocType>,
}

impl TimelineEventContentTwelveOneDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentTwelveOneDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveOneDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentTwelveOneDocBuilder::content)
    /// - [`r#type`](TimelineEventContentTwelveOneDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDoc, BuildError> {
        Ok(TimelineEventContentTwelveOneDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
