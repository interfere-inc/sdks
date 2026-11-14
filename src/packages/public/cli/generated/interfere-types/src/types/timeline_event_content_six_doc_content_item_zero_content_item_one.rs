pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemOneAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemOneType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemOne {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemOneBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemOneBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemOneAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemOneType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemOne, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
