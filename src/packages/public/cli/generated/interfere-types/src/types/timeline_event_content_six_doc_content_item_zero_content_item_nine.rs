pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemNine {
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemNineAttrs,
}

impl TimelineEventContentSixDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemNineBuilder {
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemNineType>,
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemNineAttrs>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
