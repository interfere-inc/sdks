pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemFourAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemFourType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemFour {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFourAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFourType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemFour, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
