pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne {
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder {
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder {
    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder::doc)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
