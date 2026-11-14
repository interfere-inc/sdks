pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentSixDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentSixDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentSixDocContentItemOneType,
}

impl TimelineEventContentSixDocContentItemOne {
    pub fn builder() -> TimelineEventContentSixDocContentItemOneBuilder {
        <TimelineEventContentSixDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentSixDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentSixDocContentItemOneType>,
}

impl TimelineEventContentSixDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentSixDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentSixDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemOne, BuildError> {
        Ok(TimelineEventContentSixDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
