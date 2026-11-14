pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineProvider,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineProvider>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
