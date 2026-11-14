pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
