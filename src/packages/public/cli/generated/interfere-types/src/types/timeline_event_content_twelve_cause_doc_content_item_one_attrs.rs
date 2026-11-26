pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentTwelveCauseDocContentItemOneAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<TimelineEventContentTwelveCauseDocContentItemOneAttrsProvenance>,
    #[serde(rename = "lineRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneAttrs {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemOneAttrsBuilder {
        <TimelineEventContentTwelveCauseDocContentItemOneAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemOneAttrsBuilder {
    annotations: Option<Vec<TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem>>,
    language: Option<String>,
    file: Option<String>,
    provenance: Option<TimelineEventContentTwelveCauseDocContentItemOneAttrsProvenance>,
    line_range: Option<String>,
    focus: Option<String>,
    url: Option<String>,
}

impl TimelineEventContentTwelveCauseDocContentItemOneAttrsBuilder {
    pub fn annotations(mut self, value: Vec<TimelineEventContentTwelveCauseDocContentItemOneAttrsAnnotationsItem>) -> Self {
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

    pub fn provenance(mut self, value: TimelineEventContentTwelveCauseDocContentItemOneAttrsProvenance) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemOneAttrs`].
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemOneAttrs, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemOneAttrs {
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
