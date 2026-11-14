pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
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

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
    target_id: Option<String>,
    connection_id: Option<String>,
    datasource_id: Option<String>,
    connection_revision: Option<i64>,
    verified_at: Option<i64>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_id`](TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder::target_id)
    /// - [`connection_id`](TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder::connection_id)
    /// - [`datasource_id`](TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder::datasource_id)
    /// - [`connection_revision`](TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder::connection_revision)
    /// - [`verified_at`](TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionIdBuilder::verified_at)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId {
            target_id: self.target_id.ok_or_else(|| BuildError::missing_field("target_id"))?,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            verified_at: self.verified_at.ok_or_else(|| BuildError::missing_field("verified_at"))?,
        })
    }
}
