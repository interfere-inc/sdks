pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
