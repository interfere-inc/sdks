pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentFourDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<TiptapMark>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TimelineEventContentFourDocContentItemOneContentItemType,
}

impl TimelineEventContentFourDocContentItemOneContentItem {
    pub fn builder() -> TimelineEventContentFourDocContentItemOneContentItemBuilder {
        <TimelineEventContentFourDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemOneContentItemBuilder {
    marks: Option<Vec<TiptapMark>>,
    text: Option<String>,
    r#type: Option<TimelineEventContentFourDocContentItemOneContentItemType>,
}

impl TimelineEventContentFourDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: Vec<TiptapMark>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemOneContentItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemOneContentItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentFourDocContentItemOneContentItemBuilder::text)
    /// - [`r#type`](TimelineEventContentFourDocContentItemOneContentItemBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemOneContentItem, BuildError> {
        Ok(TimelineEventContentFourDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
