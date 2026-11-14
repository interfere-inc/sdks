pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSeven {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemSevenType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSeven {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemSevenType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemSevenType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemSevenBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemSeven, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemSeven {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
