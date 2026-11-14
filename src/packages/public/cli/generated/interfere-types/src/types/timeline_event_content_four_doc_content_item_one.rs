pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentFourDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentFourDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentFourDocContentItemOneType,
}

impl TimelineEventContentFourDocContentItemOne {
    pub fn builder() -> TimelineEventContentFourDocContentItemOneBuilder {
        <TimelineEventContentFourDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentFourDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentFourDocContentItemOneType>,
}

impl TimelineEventContentFourDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentFourDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentFourDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemOne, BuildError> {
        Ok(TimelineEventContentFourDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
