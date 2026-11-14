pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemTwoAttrsBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs`].
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemTwoAttrs {
            name: self.name,
        })
    }
}
