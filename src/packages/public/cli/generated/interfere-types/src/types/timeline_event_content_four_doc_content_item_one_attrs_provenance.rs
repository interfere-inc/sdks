pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentFourDocContentItemOneAttrsProvenance {
    #[serde(rename = "datasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<serde_json::Value>,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<serde_json::Value>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<serde_json::Value>,
    #[serde(rename = "observedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<serde_json::Value>,
    #[serde(rename = "retrievedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<serde_json::Value>,
    #[serde(rename = "integrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<serde_json::Value>,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub repository: String,
}

impl TimelineEventContentFourDocContentItemOneAttrsProvenance {
    pub fn builder() -> TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder {
        <TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder {
    datasource_id: Option<serde_json::Value>,
    connection_id: Option<serde_json::Value>,
    external_id: Option<serde_json::Value>,
    observed_at: Option<serde_json::Value>,
    retrieved_at: Option<serde_json::Value>,
    integration_id: Option<serde_json::Value>,
    r#ref: Option<String>,
    repository: Option<String>,
}

impl TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder {
    pub fn datasource_id(mut self, value: serde_json::Value) -> Self {
        self.datasource_id = Some(value);
        self
    }

    pub fn connection_id(mut self, value: serde_json::Value) -> Self {
        self.connection_id = Some(value);
        self
    }

    pub fn external_id(mut self, value: serde_json::Value) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn observed_at(mut self, value: serde_json::Value) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn retrieved_at(mut self, value: serde_json::Value) -> Self {
        self.retrieved_at = Some(value);
        self
    }

    pub fn integration_id(mut self, value: serde_json::Value) -> Self {
        self.integration_id = Some(value);
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemOneAttrsProvenance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#ref`](TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder::r#ref)
    /// - [`repository`](TimelineEventContentFourDocContentItemOneAttrsProvenanceBuilder::repository)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemOneAttrsProvenance, BuildError> {
        Ok(TimelineEventContentFourDocContentItemOneAttrsProvenance {
            datasource_id: self.datasource_id,
            connection_id: self.connection_id,
            external_id: self.external_id,
            observed_at: self.observed_at,
            retrieved_at: self.retrieved_at,
            integration_id: self.integration_id,
            r#ref: self.r#ref.ok_or_else(|| BuildError::missing_field("r#ref"))?,
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
        })
    }
}
