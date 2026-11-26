pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrsBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<serde_json::Value>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs`].
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemTwoAttrs {
            name: self.name,
        })
    }
}
