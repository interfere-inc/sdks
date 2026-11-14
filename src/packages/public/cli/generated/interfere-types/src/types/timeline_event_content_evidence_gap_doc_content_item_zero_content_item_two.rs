pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs,
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder {
    attrs: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
