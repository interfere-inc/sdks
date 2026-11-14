pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "confirmationId")]
    #[serde(default)]
    pub confirmation_id: String,
    #[serde(default)]
    pub statement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdMetadata>,
    #[serde(rename = "factIds")]
    #[serde(default)]
    pub fact_ids: Vec<String>,
    #[serde(rename = "factDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_digests: Option<HashMap<String, String>>,
    #[serde(rename = "observedWindow")]
    #[serde(default)]
    pub observed_window: QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdObservedWindow,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder {
    version: Option<String>,
    confirmation_id: Option<String>,
    statement: Option<String>,
    metadata: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdMetadata>,
    fact_ids: Option<Vec<String>>,
    fact_digests: Option<HashMap<String, String>>,
    observed_window: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdObservedWindow>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder {
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

    pub fn metadata(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdMetadata) -> Self {
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

    pub fn observed_window(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdObservedWindow) -> Self {
        self.observed_window = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder::version)
    /// - [`confirmation_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder::confirmation_id)
    /// - [`statement`](QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder::statement)
    /// - [`fact_ids`](QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder::fact_ids)
    /// - [`observed_window`](QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationIdBuilder::observed_window)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId {
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
