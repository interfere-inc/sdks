pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
