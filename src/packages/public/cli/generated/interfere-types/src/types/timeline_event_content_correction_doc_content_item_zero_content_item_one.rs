pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemOneAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemOneType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemOne {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemOneAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemOneType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemOne, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
