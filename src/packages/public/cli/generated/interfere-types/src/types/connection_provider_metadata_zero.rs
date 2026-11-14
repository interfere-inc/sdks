pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataZero {
    pub provider: ConnectionProviderMetadataZeroProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
}

impl ConnectionProviderMetadataZero {
    pub fn builder() -> ConnectionProviderMetadataZeroBuilder {
        <ConnectionProviderMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataZeroBuilder {
    provider: Option<ConnectionProviderMetadataZeroProvider>,
    name: Option<String>,
    installation_url: Option<String>,
}

impl ConnectionProviderMetadataZeroBuilder {
    pub fn provider(mut self, value: ConnectionProviderMetadataZeroProvider) -> Self {
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

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](ConnectionProviderMetadataZeroBuilder::provider)
    /// - [`name`](ConnectionProviderMetadataZeroBuilder::name)
    pub fn build(self) -> Result<ConnectionProviderMetadataZero, BuildError> {
        Ok(ConnectionProviderMetadataZero {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            installation_url: self.installation_url,
        })
    }
}
