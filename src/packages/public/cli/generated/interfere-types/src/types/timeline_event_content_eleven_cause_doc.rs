pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenCauseDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentElevenCauseDocContentItem>,
    pub r#type: TimelineEventContentElevenCauseDocType,
}

impl TimelineEventContentElevenCauseDoc {
    pub fn builder() -> TimelineEventContentElevenCauseDocBuilder {
        <TimelineEventContentElevenCauseDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocBuilder {
    content: Option<Vec<TimelineEventContentElevenCauseDocContentItem>>,
    r#type: Option<TimelineEventContentElevenCauseDocType>,
}

impl TimelineEventContentElevenCauseDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentElevenCauseDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenCauseDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentElevenCauseDocBuilder::content)
    /// - [`r#type`](TimelineEventContentElevenCauseDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDoc, BuildError> {
        Ok(TimelineEventContentElevenCauseDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
