pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenCauseDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentElevenCauseDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentElevenCauseDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentElevenCauseDocContentItemOneType,
}

impl TimelineEventContentElevenCauseDocContentItemOne {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemOneBuilder {
        <TimelineEventContentElevenCauseDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentElevenCauseDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentElevenCauseDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentElevenCauseDocContentItemOneType>,
}

impl TimelineEventContentElevenCauseDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentElevenCauseDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentElevenCauseDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenCauseDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentElevenCauseDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemOne, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
