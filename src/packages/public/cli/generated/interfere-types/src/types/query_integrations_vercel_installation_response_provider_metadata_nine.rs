pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsVercelInstallationResponseProviderMetadataNineProvider,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataNineBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataNineProvider>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsVercelInstallationResponseProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
