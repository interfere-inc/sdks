pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveCauseDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentTwelveCauseDocContentItem>,
    pub r#type: TimelineEventContentTwelveCauseDocType,
}

impl TimelineEventContentTwelveCauseDoc {
    pub fn builder() -> TimelineEventContentTwelveCauseDocBuilder {
        <TimelineEventContentTwelveCauseDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocBuilder {
    content: Option<Vec<TimelineEventContentTwelveCauseDocContentItem>>,
    r#type: Option<TimelineEventContentTwelveCauseDocType>,
}

impl TimelineEventContentTwelveCauseDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentTwelveCauseDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveCauseDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentTwelveCauseDocBuilder::content)
    /// - [`r#type`](TimelineEventContentTwelveCauseDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDoc, BuildError> {
        Ok(TimelineEventContentTwelveCauseDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
