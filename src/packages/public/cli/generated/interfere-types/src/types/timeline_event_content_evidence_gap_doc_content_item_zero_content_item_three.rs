pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree {
    pub r#type: TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeType,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeType>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentEvidenceGapDocContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
