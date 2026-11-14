pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemSevenAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemSevenType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSeven {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemSevenType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemSeven, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
