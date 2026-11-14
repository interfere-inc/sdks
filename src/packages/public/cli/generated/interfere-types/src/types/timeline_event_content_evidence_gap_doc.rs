pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDoc {
    #[serde(default)]
    pub content: Vec<TimelineEventContentEvidenceGapDocContentItem>,
    pub r#type: TimelineEventContentEvidenceGapDocType,
}

impl TimelineEventContentEvidenceGapDoc {
    pub fn builder() -> TimelineEventContentEvidenceGapDocBuilder {
        <TimelineEventContentEvidenceGapDocBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocBuilder {
    content: Option<Vec<TimelineEventContentEvidenceGapDocContentItem>>,
    r#type: Option<TimelineEventContentEvidenceGapDocType>,
}

impl TimelineEventContentEvidenceGapDocBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentEvidenceGapDocContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDoc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](TimelineEventContentEvidenceGapDocBuilder::content)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDoc, BuildError> {
        Ok(TimelineEventContentEvidenceGapDoc {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
