pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs,
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemTwoType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemTwo {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemTwoType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemTwo, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
