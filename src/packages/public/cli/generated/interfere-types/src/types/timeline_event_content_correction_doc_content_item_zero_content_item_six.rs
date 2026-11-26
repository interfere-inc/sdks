pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemSixType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSix {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSixType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemSix, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
