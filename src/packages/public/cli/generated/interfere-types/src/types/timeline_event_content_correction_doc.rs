pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentCorrectionDocContentItem>,
    pub r#type: TimelineEventContentCorrectionDocType,
}

impl TimelineEventContentCorrectionDoc {
    pub fn builder() -> TimelineEventContentCorrectionDocBuilder {
        <TimelineEventContentCorrectionDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocBuilder {
    content: Option<Vec<TimelineEventContentCorrectionDocContentItem>>,
    r#type: Option<TimelineEventContentCorrectionDocType>,
}

impl TimelineEventContentCorrectionDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentCorrectionDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentCorrectionDocBuilder::content)
    /// - [`r#type`](TimelineEventContentCorrectionDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDoc, BuildError> {
        Ok(TimelineEventContentCorrectionDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
