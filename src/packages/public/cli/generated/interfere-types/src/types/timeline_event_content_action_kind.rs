pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentActionKind {
    #[serde(rename = "actionKind")]
    pub action_kind: TimelineEventContentActionKindActionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<TimelineEventContentActionKindRepository>,
    #[serde(rename = "repositoryAccessReadyAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_access_ready_at: Option<i64>,
    #[serde(rename = "requiredEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_evidence: Option<TimelineEventContentActionKindRequiredEvidence>,
    #[serde(rename = "evidenceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_status: Option<TimelineEventContentActionKindEvidenceStatus>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "mcpConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_connection: Option<TimelineEventContentActionKindMcpConnection>,
    #[serde(rename = "mcpReadinessConsumed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_readiness_consumed: Option<TimelineEventContentActionKindMcpReadinessConsumed>,
    #[serde(rename = "mcpSetupWithdrawal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_setup_withdrawal: Option<TimelineEventContentActionKindMcpSetupWithdrawal>,
    #[serde(rename = "gapKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_kind: Option<TimelineEventContentActionKindGapKind>,
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "docsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<TimelineEventContentActionKindSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<TimelineEventContentActionKindOptionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub question: String,
    #[serde(rename = "respondedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub responded_at: Option<f64>,
    #[serde(rename = "selectedOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_id: Option<String>,
    #[serde(rename = "selectedOptionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_ids: Option<Vec<String>>,
}

impl TimelineEventContentActionKind {
    pub fn builder() -> TimelineEventContentActionKindBuilder {
        <TimelineEventContentActionKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindBuilder {
    action_kind: Option<TimelineEventContentActionKindActionKind>,
    repository: Option<TimelineEventContentActionKindRepository>,
    repository_access_ready_at: Option<i64>,
    required_evidence: Option<TimelineEventContentActionKindRequiredEvidence>,
    evidence_status: Option<TimelineEventContentActionKindEvidenceStatus>,
    target_id: Option<String>,
    mcp_connection: Option<TimelineEventContentActionKindMcpConnection>,
    mcp_readiness_consumed: Option<TimelineEventContentActionKindMcpReadinessConsumed>,
    mcp_setup_withdrawal: Option<TimelineEventContentActionKindMcpSetupWithdrawal>,
    gap_kind: Option<TimelineEventContentActionKindGapKind>,
    integration_name: Option<String>,
    docs_url: Option<String>,
    surface: Option<TimelineEventContentActionKindSurface>,
    answer: Option<String>,
    options: Option<Vec<TimelineEventContentActionKindOptionsItem>>,
    placeholder: Option<String>,
    question: Option<String>,
    responded_at: Option<f64>,
    selected_option_id: Option<String>,
    selected_option_ids: Option<Vec<String>>,
}

impl TimelineEventContentActionKindBuilder {
    pub fn action_kind(mut self, value: TimelineEventContentActionKindActionKind) -> Self {
        self.action_kind = Some(value);
        self
    }

    pub fn repository(mut self, value: TimelineEventContentActionKindRepository) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn repository_access_ready_at(mut self, value: i64) -> Self {
        self.repository_access_ready_at = Some(value);
        self
    }

    pub fn required_evidence(mut self, value: TimelineEventContentActionKindRequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn evidence_status(mut self, value: TimelineEventContentActionKindEvidenceStatus) -> Self {
        self.evidence_status = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn mcp_connection(mut self, value: TimelineEventContentActionKindMcpConnection) -> Self {
        self.mcp_connection = Some(value);
        self
    }

    pub fn mcp_readiness_consumed(mut self, value: TimelineEventContentActionKindMcpReadinessConsumed) -> Self {
        self.mcp_readiness_consumed = Some(value);
        self
    }

    pub fn mcp_setup_withdrawal(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawal) -> Self {
        self.mcp_setup_withdrawal = Some(value);
        self
    }

    pub fn gap_kind(mut self, value: TimelineEventContentActionKindGapKind) -> Self {
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

    pub fn surface(mut self, value: TimelineEventContentActionKindSurface) -> Self {
        self.surface = Some(value);
        self
    }

    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<TimelineEventContentActionKindOptionsItem>) -> Self {
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

    pub fn responded_at(mut self, value: f64) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentActionKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action_kind`](TimelineEventContentActionKindBuilder::action_kind)
    /// - [`question`](TimelineEventContentActionKindBuilder::question)
    pub fn build(self) -> Result<TimelineEventContentActionKind, BuildError> {
        Ok(TimelineEventContentActionKind {
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
