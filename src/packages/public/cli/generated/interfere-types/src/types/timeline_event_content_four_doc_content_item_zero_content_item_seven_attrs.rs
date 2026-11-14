pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemSevenAttrsBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemSevenAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemSevenAttrsBuilder {
    line: Option<serde_json::Value>,
    path: Option<serde_json::Value>,
    r#ref: Option<serde_json::Value>,
    repository: Option<serde_json::Value>,
    url: Option<serde_json::Value>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemSevenAttrsBuilder {
    pub fn line(mut self, value: serde_json::Value) -> Self {
        self.line = Some(value);
        self
    }

    pub fn path(mut self, value: serde_json::Value) -> Self {
        self.path = Some(value);
        self
    }

    pub fn r#ref(mut self, value: serde_json::Value) -> Self {
        self.r#ref = Some(value);
        self
    }

    pub fn repository(mut self, value: serde_json::Value) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn url(mut self, value: serde_json::Value) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs`].
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemSevenAttrs {
            line: self.line,
            path: self.path,
            r#ref: self.r#ref,
            repository: self.repository,
            url: self.url,
        })
    }
}
