pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind {
    #[serde(rename = "actionKind")]
    pub action_kind: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindActionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "repositoryAccessReadyAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_access_ready_at: Option<i64>,
    #[serde(rename = "requiredEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence>,
    #[serde(rename = "evidenceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatus>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "mcpConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_connection: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnection>,
    #[serde(rename = "mcpReadinessConsumed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_readiness_consumed: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpReadinessConsumed>,
    #[serde(rename = "mcpSetupWithdrawal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_setup_withdrawal: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal>,
    #[serde(rename = "gapKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindGapKind>,
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "docsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub question: String,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "respondedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responded_at: Option<i64>,
    #[serde(rename = "selectedOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_id: Option<String>,
    #[serde(rename = "selectedOptionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_ids: Option<Vec<String>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder {
    action_kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindActionKind>,
    repository: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository>,
    repository_access_ready_at: Option<i64>,
    required_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence>,
    evidence_status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatus>,
    target_id: Option<String>,
    mcp_connection: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnection>,
    mcp_readiness_consumed: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpReadinessConsumed>,
    mcp_setup_withdrawal: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal>,
    gap_kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindGapKind>,
    integration_name: Option<String>,
    docs_url: Option<String>,
    service: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService>,
    surface: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindSurface>,
    answer: Option<String>,
    options: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem>>,
    placeholder: Option<String>,
    question: Option<String>,
    responded_at: Option<i64>,
    selected_option_id: Option<String>,
    selected_option_ids: Option<Vec<String>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder {
    pub fn action_kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindActionKind) -> Self {
        self.action_kind = Some(value);
        self
    }

    pub fn repository(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn repository_access_ready_at(mut self, value: i64) -> Self {
        self.repository_access_ready_at = Some(value);
        self
    }

    pub fn required_evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn evidence_status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatus) -> Self {
        self.evidence_status = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn mcp_connection(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnection) -> Self {
        self.mcp_connection = Some(value);
        self
    }

    pub fn mcp_readiness_consumed(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpReadinessConsumed) -> Self {
        self.mcp_readiness_consumed = Some(value);
        self
    }

    pub fn mcp_setup_withdrawal(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal) -> Self {
        self.mcp_setup_withdrawal = Some(value);
        self
    }

    pub fn gap_kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindGapKind) -> Self {
        self.gap_kind = Some(value);
        self
    }

    pub fn integration_name(mut self, value: impl Into<String>) -> Self {
        self.integration_name = Some(value.into());
        self
    }

    pub fn docs_url(mut self, value: impl Into<String>) -> Self {
        self.docs_url = Some(value.into());
        self
    }

    pub fn service(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService) -> Self {
        self.service = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindSurface) -> Self {
        self.surface = Some(value);
        self
    }

    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    pub fn question(mut self, value: impl Into<String>) -> Self {
        self.question = Some(value.into());
        self
    }

    pub fn responded_at(mut self, value: i64) -> Self {
        self.responded_at = Some(value);
        self
    }

    pub fn selected_option_id(mut self, value: impl Into<String>) -> Self {
        self.selected_option_id = Some(value.into());
        self
    }

    pub fn selected_option_ids(mut self, value: Vec<String>) -> Self {
        self.selected_option_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action_kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder::action_kind)
    /// - [`question`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindBuilder::question)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKind {
            action_kind: self.action_kind.ok_or_else(|| BuildError::missing_field("action_kind"))?,
            repository: self.repository,
            repository_access_ready_at: self.repository_access_ready_at,
            required_evidence: self.required_evidence,
            evidence_status: self.evidence_status,
            target_id: self.target_id,
            mcp_connection: self.mcp_connection,
            mcp_readiness_consumed: self.mcp_readiness_consumed,
            mcp_setup_withdrawal: self.mcp_setup_withdrawal,
            gap_kind: self.gap_kind,
            integration_name: self.integration_name,
            docs_url: self.docs_url,
            service: self.service,
            surface: self.surface,
            answer: self.answer,
            options: self.options,
            placeholder: self.placeholder,
            question: self.question.ok_or_else(|| BuildError::missing_field("question"))?,
            responded_at: self.responded_at,
            selected_option_id: self.selected_option_id,
            selected_option_ids: self.selected_option_ids,
        })
    }
}
