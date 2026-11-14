pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesDestroyResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesDestroyResponse {
    pub fn builder() -> MutationWorkspacesDestroyResponseBuilder {
        <MutationWorkspacesDestroyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesDestroyResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesDestroyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesDestroyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesDestroyResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesDestroyResponse, BuildError> {
        Ok(MutationWorkspacesDestroyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
