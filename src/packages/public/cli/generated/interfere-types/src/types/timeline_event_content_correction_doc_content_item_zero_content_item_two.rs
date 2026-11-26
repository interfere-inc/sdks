pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemTwoType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemTwo {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemTwoType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemTwo, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
