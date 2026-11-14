pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RepositoriesIntegrationsRequest {
    #[serde(default)]
    pub args: RepositoriesIntegrationsRequestArgs,
}

impl RepositoriesIntegrationsRequest {
    pub fn builder() -> RepositoriesIntegrationsRequestBuilder {
        <RepositoriesIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RepositoriesIntegrationsRequestBuilder {
    args: Option<RepositoriesIntegrationsRequestArgs>,
}

impl RepositoriesIntegrationsRequestBuilder {
    pub fn args(mut self, value: RepositoriesIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RepositoriesIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RepositoriesIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<RepositoriesIntegrationsRequest, BuildError> {
        Ok(RepositoriesIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

