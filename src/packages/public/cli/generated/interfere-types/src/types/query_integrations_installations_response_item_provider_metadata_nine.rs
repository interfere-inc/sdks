pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsInstallationsResponseItemProviderMetadataNineProvider,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataNineBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataNineProvider>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsInstallationsResponseItemProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
