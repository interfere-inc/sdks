pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentElevenOneDocContentItemOneAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<TimelineEventContentElevenOneDocContentItemOneAttrsAnnotationsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<TimelineEventContentElevenOneDocContentItemOneAttrsProvenance>,
    #[serde(rename = "lineRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TimelineEventContentElevenOneDocContentItemOneAttrs {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemOneAttrsBuilder {
        <TimelineEventContentElevenOneDocContentItemOneAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemOneAttrsBuilder {
    annotations: Option<Vec<TimelineEventContentElevenOneDocContentItemOneAttrsAnnotationsItem>>,
    language: Option<String>,
    file: Option<String>,
    provenance: Option<TimelineEventContentElevenOneDocContentItemOneAttrsProvenance>,
    line_range: Option<String>,
    focus: Option<String>,
    url: Option<String>,
}

impl TimelineEventContentElevenOneDocContentItemOneAttrsBuilder {
    pub fn annotations(mut self, value: Vec<TimelineEventContentElevenOneDocContentItemOneAttrsAnnotationsItem>) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    pub fn provenance(mut self, value: TimelineEventContentElevenOneDocContentItemOneAttrsProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn line_range(mut self, value: impl Into<String>) -> Self {
        self.line_range = Some(value.into());
        self
    }

    pub fn focus(mut self, value: impl Into<String>) -> Self {
        self.focus = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemOneAttrs`].
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemOneAttrs, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemOneAttrs {
            annotations: self.annotations,
            language: self.language,
            file: self.file,
            provenance: self.provenance,
            line_range: self.line_range,
            focus: self.focus,
            url: self.url,
        })
    }
}
