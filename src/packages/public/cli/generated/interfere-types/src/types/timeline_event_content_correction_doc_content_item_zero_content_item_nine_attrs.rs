pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrsBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrsBuilder {
    value: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrsBuilder {
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs`].
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemNineAttrs {
            value: self.value,
        })
    }
}
