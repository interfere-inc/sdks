pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne {
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder {
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder {
    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder::doc)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
