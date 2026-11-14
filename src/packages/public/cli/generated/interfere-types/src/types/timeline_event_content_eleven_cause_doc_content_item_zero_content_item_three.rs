pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct TimelineEventContentElevenCauseDocContentItemZeroContentItemThree {
    pub r#type: Option<serde_json::Value>,
}

impl TimelineEventContentElevenCauseDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentElevenCauseDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<serde_json::Value>,
}

impl TimelineEventContentElevenCauseDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemZeroContentItemThree`].
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemZeroContentItemThree {
            r#type: self.r#type,
        })
    }
}
