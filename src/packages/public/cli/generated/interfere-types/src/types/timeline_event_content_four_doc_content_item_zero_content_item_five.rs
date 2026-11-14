pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFive {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemFiveAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemFiveType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFive {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemFiveType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemFive, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
