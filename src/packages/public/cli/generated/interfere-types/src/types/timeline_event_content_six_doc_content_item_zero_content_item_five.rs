pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFive {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemFiveType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFive {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemFiveType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemFive, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
