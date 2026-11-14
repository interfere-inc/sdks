pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponseSameRequestItem {
    #[serde(default)]
    pub closed: bool,
    #[serde(rename = "datasourceProblemId")]
    #[serde(default)]
    pub datasource_problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ProblemRelatedResponseSameRequestItemEvidence>,
    #[serde(rename = "sampleDatasourceSpanId")]
    #[serde(default)]
    pub sample_datasource_span_id: String,
    #[serde(rename = "sampleFactId")]
    #[serde(default)]
    pub sample_fact_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    #[serde(rename = "sharedRequests")]
    #[serde(default)]
    pub shared_requests: i64,
    pub status: ProblemRelatedResponseSameRequestItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ProblemRelatedResponseSameRequestItem {
    pub fn builder() -> ProblemRelatedResponseSameRequestItemBuilder {
        <ProblemRelatedResponseSameRequestItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponseSameRequestItemBuilder {
    closed: Option<bool>,
    datasource_problem_id: Option<String>,
    description: Option<String>,
    evidence: Option<ProblemRelatedResponseSameRequestItemEvidence>,
    sample_datasource_span_id: Option<String>,
    sample_fact_id: Option<String>,
    sequence: Option<i64>,
    shared_requests: Option<i64>,
    status: Option<ProblemRelatedResponseSameRequestItemStatus>,
    title: Option<String>,
}

impl ProblemRelatedResponseSameRequestItemBuilder {
    pub fn closed(mut self, value: bool) -> Self {
        self.closed = Some(value);
        self
    }

    pub fn datasource_problem_id(mut self, value: impl Into<String>) -> Self {
        self.datasource_problem_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn evidence(mut self, value: ProblemRelatedResponseSameRequestItemEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn sample_datasource_span_id(mut self, value: impl Into<String>) -> Self {
        self.sample_datasource_span_id = Some(value.into());
        self
    }

    pub fn sample_fact_id(mut self, value: impl Into<String>) -> Self {
        self.sample_fact_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn shared_requests(mut self, value: i64) -> Self {
        self.shared_requests = Some(value);
        self
    }

    pub fn status(mut self, value: ProblemRelatedResponseSameRequestItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponseSameRequestItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`closed`](ProblemRelatedResponseSameRequestItemBuilder::closed)
    /// - [`datasource_problem_id`](ProblemRelatedResponseSameRequestItemBuilder::datasource_problem_id)
    /// - [`sample_datasource_span_id`](ProblemRelatedResponseSameRequestItemBuilder::sample_datasource_span_id)
    /// - [`sample_fact_id`](ProblemRelatedResponseSameRequestItemBuilder::sample_fact_id)
    /// - [`shared_requests`](ProblemRelatedResponseSameRequestItemBuilder::shared_requests)
    /// - [`status`](ProblemRelatedResponseSameRequestItemBuilder::status)
    pub fn build(self) -> Result<ProblemRelatedResponseSameRequestItem, BuildError> {
        Ok(ProblemRelatedResponseSameRequestItem {
            closed: self.closed.ok_or_else(|| BuildError::missing_field("closed"))?,
            datasource_problem_id: self.datasource_problem_id.ok_or_else(|| BuildError::missing_field("datasource_problem_id"))?,
            description: self.description,
            evidence: self.evidence,
            sample_datasource_span_id: self.sample_datasource_span_id.ok_or_else(|| BuildError::missing_field("sample_datasource_span_id"))?,
            sample_fact_id: self.sample_fact_id.ok_or_else(|| BuildError::missing_field("sample_fact_id"))?,
            sequence: self.sequence,
            shared_requests: self.shared_requests.ok_or_else(|| BuildError::missing_field("shared_requests"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
        })
    }
}
