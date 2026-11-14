pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataZero {
    pub provider: QueryIntegrationsInstallationsResponseItemProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataZero {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder {
    provider: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataZeroProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder::provider)
    /// - [`name`](QueryIntegrationsInstallationsResponseItemProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataZero, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
