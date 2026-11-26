pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveOneDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentTwelveOneDocContentItemOneContentItemType,
}

impl TimelineEventContentTwelveOneDocContentItemOneContentItem {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder {
        <TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder {
    marks: Option<Vec<Option<serde_json::Value>>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentTwelveOneDocContentItemOneContentItemType>,
}

impl TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveOneDocContentItemOneContentItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemOneContentItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder::text)
    /// - [`r#type`](TimelineEventContentTwelveOneDocContentItemOneContentItemBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemOneContentItem, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
