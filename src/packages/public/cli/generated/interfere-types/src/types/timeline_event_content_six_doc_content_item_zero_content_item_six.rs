pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSix {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemSixAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemSixType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSix {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemSixBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSixBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemSixAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemSixType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSixBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemSixAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemSixType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemSixBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemSixBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemSix, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemSix {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
