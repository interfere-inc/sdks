pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemEightType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemEight {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemEightType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemEight, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
