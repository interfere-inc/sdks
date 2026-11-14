pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataZero {
    pub provider: QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataZero {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder {
    provider: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder::provider)
    /// - [`name`](QueryIntegrationsGheServerInstallationResponseProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataZero, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
