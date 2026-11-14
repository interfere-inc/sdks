pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedInstallationsIntegrationsRequestArgs {
    #[serde(default)]
    pub providers: Vec<String>,
}

impl ConnectedInstallationsIntegrationsRequestArgs {
    pub fn builder() -> ConnectedInstallationsIntegrationsRequestArgsBuilder {
        <ConnectedInstallationsIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedInstallationsIntegrationsRequestArgsBuilder {
    providers: Option<Vec<String>>,
}

impl ConnectedInstallationsIntegrationsRequestArgsBuilder {
    pub fn providers(mut self, value: Vec<String>) -> Self {
        self.providers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectedInstallationsIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`providers`](ConnectedInstallationsIntegrationsRequestArgsBuilder::providers)
    pub fn build(self) -> Result<ConnectedInstallationsIntegrationsRequestArgs, BuildError> {
        Ok(ConnectedInstallationsIntegrationsRequestArgs {
            providers: self.providers.ok_or_else(|| BuildError::missing_field("providers"))?,
        })
    }
}
