pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InstallationsIntegrationsRequestArgs {
    #[serde(default)]
    pub providers: Vec<String>,
}

impl InstallationsIntegrationsRequestArgs {
    pub fn builder() -> InstallationsIntegrationsRequestArgsBuilder {
        <InstallationsIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallationsIntegrationsRequestArgsBuilder {
    providers: Option<Vec<String>>,
}

impl InstallationsIntegrationsRequestArgsBuilder {
    pub fn providers(mut self, value: Vec<String>) -> Self {
        self.providers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallationsIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`providers`](InstallationsIntegrationsRequestArgsBuilder::providers)
    pub fn build(self) -> Result<InstallationsIntegrationsRequestArgs, BuildError> {
        Ok(InstallationsIntegrationsRequestArgs {
            providers: self.providers.ok_or_else(|| BuildError::missing_field("providers"))?,
        })
    }
}
