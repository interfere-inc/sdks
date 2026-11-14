pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId {
    #[serde(rename = "datasourceId")]
    #[serde(default)]
    pub datasource_id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
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
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder {
    datasource_id: Option<String>,
    connection_id: Option<String>,
    external_id: Option<String>,
    observed_at: Option<DateTime<FixedOffset>>,
    retrieved_at: Option<DateTime<FixedOffset>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder {
    pub fn datasource_id(mut self, value: impl Into<String>) -> Self {
        self.datasource_id = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`datasource_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder::datasource_id)
    /// - [`connection_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder::connection_id)
    /// - [`external_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder::external_id)
    /// - [`observed_at`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder::observed_at)
    /// - [`retrieved_at`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionIdBuilder::retrieved_at)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId {
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            external_id: self.external_id.ok_or_else(|| BuildError::missing_field("external_id"))?,
            observed_at: self.observed_at.ok_or_else(|| BuildError::missing_field("observed_at"))?,
            retrieved_at: self.retrieved_at.ok_or_else(|| BuildError::missing_field("retrieved_at"))?,
        })
    }
}
