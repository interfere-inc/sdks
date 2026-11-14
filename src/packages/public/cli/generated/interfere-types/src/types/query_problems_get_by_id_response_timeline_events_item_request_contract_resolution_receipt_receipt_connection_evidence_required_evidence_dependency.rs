pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
    group_id: Option<serde_json::Value>,
    mode: Option<serde_json::Value>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: serde_json::Value) -> Self {
        self.group_id = Some(value);
        self
    }

    pub fn mode(mut self, value: serde_json::Value) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
            group_id: self.group_id,
            mode: self.mode,
        })
    }
}
