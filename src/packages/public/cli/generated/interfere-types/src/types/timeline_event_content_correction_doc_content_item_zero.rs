pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZero {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentCorrectionDocContentItemZeroContentItem>>,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroType,
}

impl TimelineEventContentCorrectionDocContentItemZero {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroBuilder {
    content: Option<Vec<TimelineEventContentCorrectionDocContentItemZeroContentItem>>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroBuilder {
    pub fn content(mut self, value: Vec<TimelineEventContentCorrectionDocContentItemZeroContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZero, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZero {
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
