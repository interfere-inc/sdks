pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenOneDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentElevenOneDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentElevenOneDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentElevenOneDocContentItemOneType,
}

impl TimelineEventContentElevenOneDocContentItemOne {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemOneBuilder {
        <TimelineEventContentElevenOneDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentElevenOneDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentElevenOneDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentElevenOneDocContentItemOneType>,
}

impl TimelineEventContentElevenOneDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentElevenOneDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentElevenOneDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenOneDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentElevenOneDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemOne, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
