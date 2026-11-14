pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InstallationIntegrationsRequestArgs {
    #[serde(default)]
    pub provider: String,
}

impl InstallationIntegrationsRequestArgs {
    pub fn builder() -> InstallationIntegrationsRequestArgsBuilder {
        <InstallationIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallationIntegrationsRequestArgsBuilder {
    provider: Option<String>,
}

impl InstallationIntegrationsRequestArgsBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InstallationIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](InstallationIntegrationsRequestArgsBuilder::provider)
    pub fn build(self) -> Result<InstallationIntegrationsRequestArgs, BuildError> {
        Ok(InstallationIntegrationsRequestArgs {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
