pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree {
    pub r#type: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentTwelveCauseDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree`].
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree {
            r#type: self.r#type,
        })
    }
}
