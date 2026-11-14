pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrsBuilder {
    label: Option<serde_json::Value>,
    value: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrsBuilder {
    pub fn label(mut self, value: serde_json::Value) -> Self {
        self.label = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemEightAttrs {
            label: self.label,
            value: self.value,
        })
    }
}
