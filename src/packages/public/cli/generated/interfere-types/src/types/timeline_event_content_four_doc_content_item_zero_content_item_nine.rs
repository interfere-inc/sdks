pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemNine {
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemNineType,
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemNineAttrs,
}

impl TimelineEventContentFourDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemNineBuilder {
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemNineType>,
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemNineAttrs>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemNineType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemNineAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemNineBuilder::r#type)
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemNineBuilder::attrs)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemNine {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
        })
    }
}
