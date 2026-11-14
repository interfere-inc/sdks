pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesSetScimDirectoryEnabledResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesSetScimDirectoryEnabledResponse {
    pub fn builder() -> MutationWorkspacesSetScimDirectoryEnabledResponseBuilder {
        <MutationWorkspacesSetScimDirectoryEnabledResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesSetScimDirectoryEnabledResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesSetScimDirectoryEnabledResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesSetScimDirectoryEnabledResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesSetScimDirectoryEnabledResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesSetScimDirectoryEnabledResponse, BuildError> {
        Ok(MutationWorkspacesSetScimDirectoryEnabledResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
