pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId {
    #[serde(rename = "datasourceId")]
    #[serde(default)]
    pub datasource_id: String,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(rename = "observedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub observed_at: DateTime<FixedOffset>,
    #[serde(rename = "retrievedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub retrieved_at: DateTime<FixedOffset>,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    pub integration_id: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder {
    datasource_id: Option<String>,
    external_id: Option<String>,
    observed_at: Option<DateTime<FixedOffset>>,
    retrieved_at: Option<DateTime<FixedOffset>>,
    integration_id: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder {
    pub fn datasource_id(mut self, value: impl Into<String>) -> Self {
        self.datasource_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn observed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn retrieved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.retrieved_at = Some(value);
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`datasource_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder::datasource_id)
    /// - [`external_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder::external_id)
    /// - [`observed_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder::observed_at)
    /// - [`retrieved_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder::retrieved_at)
    /// - [`integration_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationIdBuilder::integration_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId {
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            external_id: self.external_id.ok_or_else(|| BuildError::missing_field("external_id"))?,
            observed_at: self.observed_at.ok_or_else(|| BuildError::missing_field("observed_at"))?,
            retrieved_at: self.retrieved_at.ok_or_else(|| BuildError::missing_field("retrieved_at"))?,
            integration_id: self.integration_id.ok_or_else(|| BuildError::missing_field("integration_id"))?,
        })
    }
}
