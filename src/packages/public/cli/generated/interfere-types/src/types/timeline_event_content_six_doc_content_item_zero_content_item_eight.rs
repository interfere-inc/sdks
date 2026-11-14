pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemEightAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemEightType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemEight {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemEightBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemEightBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemEightAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemEightType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemEight, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
