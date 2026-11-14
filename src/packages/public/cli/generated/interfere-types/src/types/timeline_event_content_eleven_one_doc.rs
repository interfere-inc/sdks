pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenOneDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentElevenOneDocContentItem>,
    pub r#type: TimelineEventContentElevenOneDocType,
}

impl TimelineEventContentElevenOneDoc {
    pub fn builder() -> TimelineEventContentElevenOneDocBuilder {
        <TimelineEventContentElevenOneDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocBuilder {
    content: Option<Vec<TimelineEventContentElevenOneDocContentItem>>,
    r#type: Option<TimelineEventContentElevenOneDocType>,
}

impl TimelineEventContentElevenOneDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentElevenOneDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenOneDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentElevenOneDocBuilder::content)
    /// - [`r#type`](TimelineEventContentElevenOneDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenOneDoc, BuildError> {
        Ok(TimelineEventContentElevenOneDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
