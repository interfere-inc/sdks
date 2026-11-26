pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentTwelveCauseDocContentItemOneContentItemType,
}

impl TimelineEventContentTwelveCauseDocContentItemOneContentItem {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder {
        <TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder {
    marks: Option<Vec<Option<serde_json::Value>>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentTwelveCauseDocContentItemOneContentItemType>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveCauseDocContentItemOneContentItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemOneContentItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder::text)
    /// - [`r#type`](TimelineEventContentTwelveCauseDocContentItemOneContentItemBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemOneContentItem, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
