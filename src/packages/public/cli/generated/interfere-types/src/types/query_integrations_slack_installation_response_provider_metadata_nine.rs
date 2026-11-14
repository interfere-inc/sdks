pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsSlackInstallationResponseProviderMetadataNineProvider,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataNineBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataNineProvider>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsSlackInstallationResponseProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
