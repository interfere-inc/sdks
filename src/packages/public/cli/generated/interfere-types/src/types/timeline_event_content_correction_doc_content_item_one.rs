pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemOne {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<TimelineEventContentCorrectionDocContentItemOneAttrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<TimelineEventContentCorrectionDocContentItemOneContentItem>>,
    pub r#type: TimelineEventContentCorrectionDocContentItemOneType,
}

impl TimelineEventContentCorrectionDocContentItemOne {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemOneBuilder {
        <TimelineEventContentCorrectionDocContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemOneBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemOneAttrs>,
    content: Option<Vec<TimelineEventContentCorrectionDocContentItemOneContentItem>>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemOneType>,
}

impl TimelineEventContentCorrectionDocContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn content(mut self, value: Vec<TimelineEventContentCorrectionDocContentItemOneContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemOne, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemOne {
            attrs: self.attrs,
            content: self.content,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
