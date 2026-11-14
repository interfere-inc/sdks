pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindMcpReadinessConsumedConnectionId {
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "datasourceId")]
    #[serde(default)]
    pub datasource_id: String,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    #[serde(rename = "verifiedAt")]
    #[serde(default)]
    pub verified_at: i64,
}

impl TimelineEventContentActionKindMcpReadinessConsumedConnectionId {
    pub fn builder() -> TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder {
        <TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder {
    target_id: Option<String>,
    connection_id: Option<String>,
    datasource_id: Option<String>,
    connection_revision: Option<i64>,
    verified_at: Option<i64>,
}

impl TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder {
    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpReadinessConsumedConnectionId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_id`](TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder::target_id)
    /// - [`connection_id`](TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder::connection_id)
    /// - [`datasource_id`](TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder::datasource_id)
    /// - [`connection_revision`](TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder::connection_revision)
    /// - [`verified_at`](TimelineEventContentActionKindMcpReadinessConsumedConnectionIdBuilder::verified_at)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpReadinessConsumedConnectionId, BuildError> {
        Ok(TimelineEventContentActionKindMcpReadinessConsumedConnectionId {
            target_id: self.target_id.ok_or_else(|| BuildError::missing_field("target_id"))?,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            verified_at: self.verified_at.ok_or_else(|| BuildError::missing_field("verified_at"))?,
        })
    }
}
