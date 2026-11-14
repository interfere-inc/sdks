pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemOneAttrsProvenance {
    #[serde(rename = "datasourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource_id: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "observedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub observed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "retrievedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub retrieved_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "integrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub repository: String,
}

impl TiptapDocumentContentItemOneAttrsProvenance {
    pub fn builder() -> TiptapDocumentContentItemOneAttrsProvenanceBuilder {
        <TiptapDocumentContentItemOneAttrsProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemOneAttrsProvenanceBuilder {
    datasource_id: Option<String>,
    connection_id: Option<String>,
    external_id: Option<String>,
    observed_at: Option<DateTime<FixedOffset>>,
    retrieved_at: Option<DateTime<FixedOffset>>,
    integration_id: Option<String>,
    r#ref: Option<String>,
    repository: Option<String>,
}

impl TiptapDocumentContentItemOneAttrsProvenanceBuilder {
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

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemOneAttrsProvenance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#ref`](TiptapDocumentContentItemOneAttrsProvenanceBuilder::r#ref)
    /// - [`repository`](TiptapDocumentContentItemOneAttrsProvenanceBuilder::repository)
    pub fn build(self) -> Result<TiptapDocumentContentItemOneAttrsProvenance, BuildError> {
        Ok(TiptapDocumentContentItemOneAttrsProvenance {
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
