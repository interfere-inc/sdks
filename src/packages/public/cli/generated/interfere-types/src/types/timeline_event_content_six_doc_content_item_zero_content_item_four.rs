pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFour {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemFourAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemFourType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFour {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemFourBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFourBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemFourAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemFourType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFourBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemFourAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemFourBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemFourBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemFour, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemFour {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
