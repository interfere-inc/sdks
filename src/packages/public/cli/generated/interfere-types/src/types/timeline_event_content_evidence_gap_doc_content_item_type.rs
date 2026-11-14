pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemTypeType,
}

impl TimelineEventContentEvidenceGapDocContentItemType {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemTypeBuilder {
        <TimelineEventContentEvidenceGapDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemTypeType>,
}

impl TimelineEventContentEvidenceGapDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemType, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
