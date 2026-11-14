pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
    #[serde(rename = "mentionSuggestionChar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_suggestion_char: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs {
    pub fn builder() -> TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrsBuilder {
        <TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrsBuilder {
    id: Option<serde_json::Value>,
    label: Option<serde_json::Value>,
    mention_suggestion_char: Option<serde_json::Value>,
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrsBuilder {
    pub fn id(mut self, value: serde_json::Value) -> Self {
        self.id = Some(value);
        self
    }

    pub fn label(mut self, value: serde_json::Value) -> Self {
        self.label = Some(value);
        self
    }

    pub fn mention_suggestion_char(mut self, value: serde_json::Value) -> Self {
        self.mention_suggestion_char = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs`].
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs, BuildError> {
        Ok(TimelineEventContentEvidenceGapDocContentItemZeroContentItemOneAttrs {
            id: self.id,
            label: self.label,
            mention_suggestion_char: self.mention_suggestion_char,
        })
    }
}
