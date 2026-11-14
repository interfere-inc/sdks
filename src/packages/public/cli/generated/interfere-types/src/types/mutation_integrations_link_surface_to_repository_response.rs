pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsLinkSurfaceToRepositoryResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsLinkSurfaceToRepositoryResponse {
    pub fn builder() -> MutationIntegrationsLinkSurfaceToRepositoryResponseBuilder {
        <MutationIntegrationsLinkSurfaceToRepositoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsLinkSurfaceToRepositoryResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsLinkSurfaceToRepositoryResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsLinkSurfaceToRepositoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsLinkSurfaceToRepositoryResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsLinkSurfaceToRepositoryResponse, BuildError> {
        Ok(MutationIntegrationsLinkSurfaceToRepositoryResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
