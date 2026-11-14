pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentSixDocContentItem>,
    pub r#type: TimelineEventContentSixDocType,
}

impl TimelineEventContentSixDoc {
    pub fn builder() -> TimelineEventContentSixDocBuilder {
        <TimelineEventContentSixDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocBuilder {
    content: Option<Vec<TimelineEventContentSixDocContentItem>>,
    r#type: Option<TimelineEventContentSixDocType>,
}

impl TimelineEventContentSixDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentSixDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentSixDocBuilder::content)
    /// - [`r#type`](TimelineEventContentSixDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDoc, BuildError> {
        Ok(TimelineEventContentSixDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
