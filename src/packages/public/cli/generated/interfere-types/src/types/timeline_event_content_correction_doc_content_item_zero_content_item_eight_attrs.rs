pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrsBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrsBuilder {
    label: Option<serde_json::Value>,
    value: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrsBuilder {
    pub fn label(mut self, value: serde_json::Value) -> Self {
        self.label = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs`].
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemEightAttrs {
            label: self.label,
            value: self.value,
        })
    }
}
