pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct TimelineEventContentElevenOneDocContentItemZeroContentItemThree {
    pub r#type: Option<serde_json::Value>,
}

impl TimelineEventContentElevenOneDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentElevenOneDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<serde_json::Value>,
}

impl TimelineEventContentElevenOneDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemZeroContentItemThree`].
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemZeroContentItemThree {
            r#type: self.r#type,
        })
    }
}
