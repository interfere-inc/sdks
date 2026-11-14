pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
