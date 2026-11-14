pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesUpdateEnterpriseConnectionResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesUpdateEnterpriseConnectionResponse {
    pub fn builder() -> MutationWorkspacesUpdateEnterpriseConnectionResponseBuilder {
        <MutationWorkspacesUpdateEnterpriseConnectionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesUpdateEnterpriseConnectionResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesUpdateEnterpriseConnectionResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesUpdateEnterpriseConnectionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesUpdateEnterpriseConnectionResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesUpdateEnterpriseConnectionResponse, BuildError> {
        Ok(MutationWorkspacesUpdateEnterpriseConnectionResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
