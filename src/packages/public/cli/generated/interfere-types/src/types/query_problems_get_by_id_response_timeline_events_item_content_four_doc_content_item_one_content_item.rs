pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItemBuilder {
    marks: Option<serde_json::Value>,
    text: Option<serde_json::Value>,
    r#type: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItemBuilder {
    pub fn marks(mut self, value: serde_json::Value) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: serde_json::Value) -> Self {
        self.text = Some(value);
        self
    }

    pub fn r#type(mut self, value: serde_json::Value) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text,
            r#type: self.r#type,
        })
    }
}
