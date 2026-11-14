pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineProvider,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineProvider>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
