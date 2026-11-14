pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenOneDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentElevenOneDocContentItemOneContentItemType,
}

impl TimelineEventContentElevenOneDocContentItemOneContentItem {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemOneContentItemBuilder {
        <TimelineEventContentElevenOneDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemOneContentItemBuilder {
    marks: Option<Vec<Option<serde_json::Value>>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentElevenOneDocContentItemOneContentItemType>,
}

impl TimelineEventContentElevenOneDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenOneDocContentItemOneContentItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemOneContentItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentElevenOneDocContentItemOneContentItemBuilder::text)
    /// - [`r#type`](TimelineEventContentElevenOneDocContentItemOneContentItemBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemOneContentItem, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
