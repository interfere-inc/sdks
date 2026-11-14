pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
