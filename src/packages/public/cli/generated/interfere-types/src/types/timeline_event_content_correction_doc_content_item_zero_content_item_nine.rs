pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemNine {
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder {
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemNineType>,
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
