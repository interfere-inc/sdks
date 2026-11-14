pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne {
    pub doc: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder {
    doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc>,
    version: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder {
    pub fn doc(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder::doc)
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneBuilder::version)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
