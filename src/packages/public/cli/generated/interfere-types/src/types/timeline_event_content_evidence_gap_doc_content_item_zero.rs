pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentEvidenceGapDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroType,
}

impl TimelineEventContentEvidenceGapDocContentItemZero {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentEvidenceGapDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentEvidenceGapDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZero, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
