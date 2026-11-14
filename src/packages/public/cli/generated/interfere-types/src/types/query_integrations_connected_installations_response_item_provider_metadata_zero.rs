pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero {
    pub provider: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder {
    provider: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder::provider)
    /// - [`name`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
