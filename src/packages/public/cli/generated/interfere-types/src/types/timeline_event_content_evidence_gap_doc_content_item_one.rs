pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentEvidenceGapDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentEvidenceGapDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemOneType,
}

impl TimelineEventContentEvidenceGapDocContentItemOne {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemOneBuilder {
        <TimelineEventContentEvidenceGapDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentEvidenceGapDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemOneType>,
}

impl TimelineEventContentEvidenceGapDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentEvidenceGapDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemOne, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
