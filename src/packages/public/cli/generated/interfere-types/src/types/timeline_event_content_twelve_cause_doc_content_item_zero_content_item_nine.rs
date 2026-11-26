pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentTwelveCauseDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemZeroContentItemNineBuilder {
    r#type: Option<serde_json::Value>,
    attrs: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: serde_json::Value) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine`].
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine {
            r#type: self.r#type,
            attrs: self.attrs,
        })
    }
}
