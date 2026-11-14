pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFour {
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFour {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder {
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder {
    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder::doc)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFour, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFour {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
