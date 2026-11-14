pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
}

impl TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<serde_json::Value>,
    side: Option<serde_json::Value>,
    text: Option<serde_json::Value>,
}

impl TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItemBuilder {
    pub fn line(mut self, value: serde_json::Value) -> Self {
        self.line = Some(value);
        self
    }

    pub fn side(mut self, value: serde_json::Value) -> Self {
        self.side = Some(value);
        self
    }

    pub fn text(mut self, value: serde_json::Value) -> Self {
        self.text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItem`].
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemOneAttrsAnnotationsItem {
            line: self.line,
            side: self.side,
            text: self.text,
        })
    }
}
