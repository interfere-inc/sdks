pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope {
    #[serde(rename = "problemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_id: Option<serde_json::Value>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<serde_json::Value>,
    #[serde(rename = "recordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<serde_json::Value>,
    #[serde(rename = "windowStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start: Option<serde_json::Value>,
    #[serde(rename = "windowEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_end: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScopeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScopeBuilder {
    problem_id: Option<serde_json::Value>,
    source_id: Option<serde_json::Value>,
    record_id: Option<serde_json::Value>,
    window_start: Option<serde_json::Value>,
    window_end: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScopeBuilder {
    pub fn problem_id(mut self, value: serde_json::Value) -> Self {
        self.problem_id = Some(value);
        self
    }

    pub fn source_id(mut self, value: serde_json::Value) -> Self {
        self.source_id = Some(value);
        self
    }

    pub fn record_id(mut self, value: serde_json::Value) -> Self {
        self.record_id = Some(value);
        self
    }

    pub fn window_start(mut self, value: serde_json::Value) -> Self {
        self.window_start = Some(value);
        self
    }

    pub fn window_end(mut self, value: serde_json::Value) -> Self {
        self.window_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope {
            problem_id: self.problem_id,
            source_id: self.source_id,
            record_id: self.record_id,
            window_start: self.window_start,
            window_end: self.window_end,
        })
    }
}
