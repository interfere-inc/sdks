pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemSixAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemSixType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSix {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemSixBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSixBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemSixAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemSixType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemSix, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
