pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataZero {
    pub provider: QueryIntegrationsInstallationResponseProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataZero {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder {
    provider: Option<QueryIntegrationsInstallationResponseProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataZeroProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder::provider)
    /// - [`name`](QueryIntegrationsInstallationResponseProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataZero, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
