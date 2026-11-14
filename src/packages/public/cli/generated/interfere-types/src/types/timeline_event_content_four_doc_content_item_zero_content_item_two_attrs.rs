pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemTwoAttrsBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs`].
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemTwoAttrs {
            name: self.name,
        })
    }
}
