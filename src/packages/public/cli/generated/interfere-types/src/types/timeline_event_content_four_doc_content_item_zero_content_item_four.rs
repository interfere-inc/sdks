pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemFourAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemFourType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFour {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemFourBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFourBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemFourAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemFourType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemFour, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
