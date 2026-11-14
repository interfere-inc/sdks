pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrsBuilder {
    value: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemNineAttrs {
            value: self.value,
        })
    }
}
