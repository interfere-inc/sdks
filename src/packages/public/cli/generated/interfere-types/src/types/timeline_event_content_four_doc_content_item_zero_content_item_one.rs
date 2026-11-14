pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemOne {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemOneAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemOneType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemOne {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemOneBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemOneBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemOneAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemOneType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemOneBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemOneAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemOneBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemOneBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemOne, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemOne {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
