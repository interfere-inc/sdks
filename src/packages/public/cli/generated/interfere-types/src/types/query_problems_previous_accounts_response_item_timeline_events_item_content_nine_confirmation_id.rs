pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "confirmationId")]
    #[serde(default)]
    pub confirmation_id: String,
    #[serde(default)]
    pub statement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdMetadata>,
    #[serde(rename = "factIds")]
    #[serde(default)]
    pub fact_ids: Vec<String>,
    #[serde(rename = "factDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_digests: Option<HashMap<String, String>>,
    #[serde(rename = "observedWindow")]
    #[serde(default)]
    pub observed_window: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdObservedWindow,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder {
    version: Option<String>,
    confirmation_id: Option<String>,
    statement: Option<String>,
    metadata: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdMetadata>,
    fact_ids: Option<Vec<String>>,
    fact_digests: Option<HashMap<String, String>>,
    observed_window: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdObservedWindow>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn confirmation_id(mut self, value: impl Into<String>) -> Self {
        self.confirmation_id = Some(value.into());
        self
    }

    pub fn statement(mut self, value: impl Into<String>) -> Self {
        self.statement = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn fact_ids(mut self, value: Vec<String>) -> Self {
        self.fact_ids = Some(value);
        self
    }

    pub fn fact_digests(mut self, value: HashMap<String, String>) -> Self {
        self.fact_digests = Some(value);
        self
    }

    pub fn observed_window(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdObservedWindow) -> Self {
        self.observed_window = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder::version)
    /// - [`confirmation_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder::confirmation_id)
    /// - [`statement`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder::statement)
    /// - [`fact_ids`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder::fact_ids)
    /// - [`observed_window`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationIdBuilder::observed_window)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            confirmation_id: self.confirmation_id.ok_or_else(|| BuildError::missing_field("confirmation_id"))?,
            statement: self.statement.ok_or_else(|| BuildError::missing_field("statement"))?,
            metadata: self.metadata,
            fact_ids: self.fact_ids.ok_or_else(|| BuildError::missing_field("fact_ids"))?,
            fact_digests: self.fact_digests,
            observed_window: self.observed_window.ok_or_else(|| BuildError::missing_field("observed_window"))?,
        })
    }
}
