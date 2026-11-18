pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationId {
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "datasourceId")]
    #[serde(default)]
    pub datasource_id: String,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "verifiedAt")]
    #[serde(default)]
    pub verified_at: i64,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    pub integration_id: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder {
    target_id: Option<String>,
    datasource_id: Option<String>,
    connection_revision: Option<i64>,
    verified_at: Option<i64>,
    integration_id: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder {
    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn datasource_id(mut self, value: impl Into<String>) -> Self {
        self.datasource_id = Some(value.into());
        self
    }

    pub fn connection_revision(mut self, value: i64) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn verified_at(mut self, value: i64) -> Self {
        self.verified_at = Some(value);
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder::target_id)
    /// - [`datasource_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder::datasource_id)
    /// - [`connection_revision`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder::connection_revision)
    /// - [`verified_at`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder::verified_at)
    /// - [`integration_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationIdBuilder::integration_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpConnectionIntegrationId {
            target_id: self.target_id.ok_or_else(|| BuildError::missing_field("target_id"))?,
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            verified_at: self.verified_at.ok_or_else(|| BuildError::missing_field("verified_at"))?,
            integration_id: self.integration_id.ok_or_else(|| BuildError::missing_field("integration_id"))?,
        })
    }
}
