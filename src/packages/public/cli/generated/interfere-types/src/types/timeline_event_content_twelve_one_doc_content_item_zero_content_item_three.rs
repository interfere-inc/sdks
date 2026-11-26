pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct TimelineEventContentTwelveOneDocContentItemZeroContentItemThree {
    pub r#type: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveOneDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentTwelveOneDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveOneDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemZeroContentItemThree`].
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemZeroContentItemThree {
            r#type: self.r#type,
        })
    }
}
