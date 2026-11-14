pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour {
    pub doc: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder {
    doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc>,
    version: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder {
    pub fn doc(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder::doc)
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourBuilder::version)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFour {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
