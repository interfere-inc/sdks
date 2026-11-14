pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RepositoriesIntegrationsRequestArgs {
    #[serde(default)]
    pub provider: String,
}

impl RepositoriesIntegrationsRequestArgs {
    pub fn builder() -> RepositoriesIntegrationsRequestArgsBuilder {
        <RepositoriesIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RepositoriesIntegrationsRequestArgsBuilder {
    provider: Option<String>,
}

impl RepositoriesIntegrationsRequestArgsBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RepositoriesIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](RepositoriesIntegrationsRequestArgsBuilder::provider)
    pub fn build(self) -> Result<RepositoriesIntegrationsRequestArgs, BuildError> {
        Ok(RepositoriesIntegrationsRequestArgs {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
