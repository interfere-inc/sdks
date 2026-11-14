pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs,
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemTwoType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemTwo {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemTwoType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemTwo, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
