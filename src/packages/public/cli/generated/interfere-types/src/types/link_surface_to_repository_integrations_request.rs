pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkSurfaceToRepositoryIntegrationsRequest {
    #[serde(default)]
    pub args: LinkSurfaceToRepositoryIntegrationsRequestArgs,
}

impl LinkSurfaceToRepositoryIntegrationsRequest {
    pub fn builder() -> LinkSurfaceToRepositoryIntegrationsRequestBuilder {
        <LinkSurfaceToRepositoryIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkSurfaceToRepositoryIntegrationsRequestBuilder {
    args: Option<LinkSurfaceToRepositoryIntegrationsRequestArgs>,
}

impl LinkSurfaceToRepositoryIntegrationsRequestBuilder {
    pub fn args(mut self, value: LinkSurfaceToRepositoryIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkSurfaceToRepositoryIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LinkSurfaceToRepositoryIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<LinkSurfaceToRepositoryIntegrationsRequest, BuildError> {
        Ok(LinkSurfaceToRepositoryIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

