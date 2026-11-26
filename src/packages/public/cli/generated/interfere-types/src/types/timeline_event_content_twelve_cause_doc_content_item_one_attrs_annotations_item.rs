pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItemBuilder {
        <TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItemBuilder {
    line: Option<serde_json::Value>,
    side: Option<serde_json::Value>,
    text: Option<serde_json::Value>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItemBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem`].
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem {
            line: self.line,
            side: self.side,
            text: self.text,
        })
    }
}
