pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentTwelveOneDocContentItemZeroContentItemNine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveOneDocContentItemZeroContentItemNine {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemZeroContentItemNineBuilder {
        <TimelineEventContentTwelveOneDocContentItemZeroContentItemNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemZeroContentItemNineBuilder {
    r#type: Option<serde_json::Value>,
    attrs: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveOneDocContentItemZeroContentItemNineBuilder {
    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn attrs(mut self, value: serde_json::Value) -> Self {
        self.attrs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemZeroContentItemNine`].
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemZeroContentItemNine, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemZeroContentItemNine {
            r#type: self.r#type,
            attrs: self.attrs,
        })
    }
}
