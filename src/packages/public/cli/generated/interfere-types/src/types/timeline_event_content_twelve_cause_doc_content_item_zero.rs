pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentTwelveCauseDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentTwelveCauseDocContentItemZeroType,
}

impl TimelineEventContentTwelveCauseDocContentItemZero {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemZeroBuilder {
        <TimelineEventContentTwelveCauseDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentTwelveCauseDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentTwelveCauseDocContentItemZeroType>,
}

impl TimelineEventContentTwelveCauseDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentTwelveCauseDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveCauseDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentTwelveCauseDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemZero, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
