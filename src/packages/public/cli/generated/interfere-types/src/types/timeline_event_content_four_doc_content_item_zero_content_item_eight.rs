pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemEight {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemEightAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemEightType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemEight {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemEightBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemEightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemEightBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemEightAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemEightType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemEightBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemEightAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemEightType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemEight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemEightBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemEightBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemEight, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemEight {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
