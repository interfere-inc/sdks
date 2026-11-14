pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesSetEnterpriseConnectionActiveResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesSetEnterpriseConnectionActiveResponse {
    pub fn builder() -> MutationWorkspacesSetEnterpriseConnectionActiveResponseBuilder {
        <MutationWorkspacesSetEnterpriseConnectionActiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesSetEnterpriseConnectionActiveResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesSetEnterpriseConnectionActiveResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesSetEnterpriseConnectionActiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesSetEnterpriseConnectionActiveResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesSetEnterpriseConnectionActiveResponse, BuildError> {
        Ok(MutationWorkspacesSetEnterpriseConnectionActiveResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
