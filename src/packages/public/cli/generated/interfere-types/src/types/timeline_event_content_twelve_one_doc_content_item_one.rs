pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveOneDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentTwelveOneDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentTwelveOneDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentTwelveOneDocContentItemOneType,
}

impl TimelineEventContentTwelveOneDocContentItemOne {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemOneBuilder {
        <TimelineEventContentTwelveOneDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentTwelveOneDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentTwelveOneDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentTwelveOneDocContentItemOneType>,
}

impl TimelineEventContentTwelveOneDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentTwelveOneDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentTwelveOneDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveOneDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentTwelveOneDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemOne, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
