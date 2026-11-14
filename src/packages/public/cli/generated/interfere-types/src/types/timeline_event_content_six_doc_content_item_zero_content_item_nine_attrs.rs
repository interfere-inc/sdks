pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemNineAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemNineAttrsBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemNineAttrsBuilder {
    value: Option<serde_json::Value>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemNineAttrs`].
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemNineAttrs {
            value: self.value,
        })
    }
}
