pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<serde_json::Value>,
    #[serde(rename = "lineRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsBuilder {
    annotations: Option<serde_json::Value>,
    language: Option<serde_json::Value>,
    file: Option<serde_json::Value>,
    provenance: Option<serde_json::Value>,
    line_range: Option<serde_json::Value>,
    focus: Option<serde_json::Value>,
    url: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrsBuilder {
    pub fn annotations(mut self, value: serde_json::Value) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn language(mut self, value: serde_json::Value) -> Self {
        self.language = Some(value);
        self
    }

    pub fn file(mut self, value: serde_json::Value) -> Self {
        self.file = Some(value);
        self
    }

    pub fn provenance(mut self, value: serde_json::Value) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn line_range(mut self, value: serde_json::Value) -> Self {
        self.line_range = Some(value);
        self
    }

    pub fn focus(mut self, value: serde_json::Value) -> Self {
        self.focus = Some(value);
        self
    }

    pub fn url(mut self, value: serde_json::Value) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneAttrs {
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
