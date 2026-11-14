pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataNine {
    #[serde(rename = "installationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_url: Option<String>,
    pub provider: ConnectionProviderMetadataNineProvider,
}

impl ConnectionProviderMetadataNine {
    pub fn builder() -> ConnectionProviderMetadataNineBuilder {
        <ConnectionProviderMetadataNineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataNineBuilder {
    installation_url: Option<String>,
    provider: Option<ConnectionProviderMetadataNineProvider>,
}

impl ConnectionProviderMetadataNineBuilder {
    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: ConnectionProviderMetadataNineProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataNine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](ConnectionProviderMetadataNineBuilder::provider)
    pub fn build(self) -> Result<ConnectionProviderMetadataNine, BuildError> {
        Ok(ConnectionProviderMetadataNine {
            installation_url: self.installation_url,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
