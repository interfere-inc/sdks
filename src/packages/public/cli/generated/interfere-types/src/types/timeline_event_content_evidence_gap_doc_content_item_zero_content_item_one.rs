pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
