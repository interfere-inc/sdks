pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: QueryIntegrationsGheServerInstallationResponseProviderMetadataNineProvider,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataNine {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataNineBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataNineProvider>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsGheServerInstallationResponseProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataNine, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
