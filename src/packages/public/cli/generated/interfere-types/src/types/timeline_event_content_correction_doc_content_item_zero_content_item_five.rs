pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemFive {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemFiveAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemFiveType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemFive {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFiveType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemFive, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
