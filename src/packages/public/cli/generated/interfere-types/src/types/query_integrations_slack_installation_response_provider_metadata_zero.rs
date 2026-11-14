pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataZero {
    pub provider: QueryIntegrationsSlackInstallationResponseProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataZero {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder {
    provider: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataZeroProvider) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder::provider)
    /// - [`name`](QueryIntegrationsSlackInstallationResponseProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataZero, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
