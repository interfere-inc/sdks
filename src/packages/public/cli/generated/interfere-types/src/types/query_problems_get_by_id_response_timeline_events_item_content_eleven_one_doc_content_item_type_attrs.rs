pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrsBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrsBuilder {
    alt: Option<serde_json::Value>,
    caption: Option<serde_json::Value>,
    source: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrsBuilder {
    pub fn alt(mut self, value: serde_json::Value) -> Self {
        self.alt = Some(value);
        self
    }

    pub fn caption(mut self, value: serde_json::Value) -> Self {
        self.caption = Some(value);
        self
    }

    pub fn source(mut self, value: serde_json::Value) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs {
            alt: self.alt,
            caption: self.caption,
            source: self.source,
        })
    }
}
