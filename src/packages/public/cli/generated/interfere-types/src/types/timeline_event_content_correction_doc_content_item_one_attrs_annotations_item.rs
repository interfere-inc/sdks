pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItem {
    #[serde(default)]
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemSide>,
    #[serde(default)]
    pub text: String,
}

impl TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<i64>,
    side: Option<TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemSide>,
    text: Option<String>,
}

impl TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line`](TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder::line)
    /// - [`text`](TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItemBuilder::text)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemOneAttrsAnnotationsItem {
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            side: self.side,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
