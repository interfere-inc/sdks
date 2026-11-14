pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentFourDocContentItem>,
    pub r#type: TimelineEventContentFourDocType,
}

impl TimelineEventContentFourDoc {
    pub fn builder() -> TimelineEventContentFourDocBuilder {
        <TimelineEventContentFourDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocBuilder {
    content: Option<Vec<TimelineEventContentFourDocContentItem>>,
    r#type: Option<TimelineEventContentFourDocType>,
}

impl TimelineEventContentFourDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentFourDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentFourDocBuilder::content)
    /// - [`r#type`](TimelineEventContentFourDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDoc, BuildError> {
        Ok(TimelineEventContentFourDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
