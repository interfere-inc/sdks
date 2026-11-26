pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemSevenAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemSevenType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSeven {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSevenType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemSeven, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
