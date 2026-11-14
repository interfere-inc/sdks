pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenCauseDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentElevenCauseDocContentItemOneContentItemType,
}

impl TimelineEventContentElevenCauseDocContentItemOneContentItem {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder {
        <TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder {
    marks: Option<Vec<Option<serde_json::Value>>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentElevenCauseDocContentItemOneContentItemType>,
}

impl TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenCauseDocContentItemOneContentItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemOneContentItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder::text)
    /// - [`r#type`](TimelineEventContentElevenCauseDocContentItemOneContentItemBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemOneContentItem, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
