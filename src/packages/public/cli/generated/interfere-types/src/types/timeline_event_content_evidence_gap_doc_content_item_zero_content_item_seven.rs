pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
