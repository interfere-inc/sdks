pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine {
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder {
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineType>,
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
