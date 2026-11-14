pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItemBuilder {
    marks: Option<serde_json::Value>,
    text: Option<serde_json::Value>,
    r#type: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItemBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOneContentItem {
            marks: self.marks,
            text: self.text,
            r#type: self.r#type,
        })
    }
}
