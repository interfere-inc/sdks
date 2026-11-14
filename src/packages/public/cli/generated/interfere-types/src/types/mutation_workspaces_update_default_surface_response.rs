pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesUpdateDefaultSurfaceResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesUpdateDefaultSurfaceResponse {
    pub fn builder() -> MutationWorkspacesUpdateDefaultSurfaceResponseBuilder {
        <MutationWorkspacesUpdateDefaultSurfaceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesUpdateDefaultSurfaceResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesUpdateDefaultSurfaceResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesUpdateDefaultSurfaceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesUpdateDefaultSurfaceResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesUpdateDefaultSurfaceResponse, BuildError> {
        Ok(MutationWorkspacesUpdateDefaultSurfaceResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
