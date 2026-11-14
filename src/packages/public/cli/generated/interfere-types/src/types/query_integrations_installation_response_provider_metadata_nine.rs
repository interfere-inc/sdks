pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsInstallationResponseProviderMetadataNineProvider,
}

impl QueryIntegrationsInstallationResponseProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataNineBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsInstallationResponseProviderMetadataNineProvider>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsInstallationResponseProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
