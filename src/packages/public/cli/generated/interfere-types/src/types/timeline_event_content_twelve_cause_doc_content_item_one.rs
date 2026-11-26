pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentTwelveCauseDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentTwelveCauseDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentTwelveCauseDocContentItemOneType,
}

impl TimelineEventContentTwelveCauseDocContentItemOne {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemOneBuilder {
        <TimelineEventContentTwelveCauseDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentTwelveCauseDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentTwelveCauseDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentTwelveCauseDocContentItemOneType>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentTwelveCauseDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentTwelveCauseDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveCauseDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentTwelveCauseDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemOne, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
