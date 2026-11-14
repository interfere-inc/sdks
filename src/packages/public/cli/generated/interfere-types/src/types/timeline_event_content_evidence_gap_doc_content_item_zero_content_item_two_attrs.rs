pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwoAttrs {
            name: self.name,
        })
    }
}
