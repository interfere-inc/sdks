pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemNineAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemNineAttrsBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemNineAttrsBuilder {
    value: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemNineAttrs`].
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemNineAttrs {
            value: self.value,
        })
    }
}
