pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationPublicKeysRevokeResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationPublicKeysRevokeResponse {
    pub fn builder() -> MutationPublicKeysRevokeResponseBuilder {
        <MutationPublicKeysRevokeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationPublicKeysRevokeResponseBuilder {
    success: Option<bool>,
}

impl MutationPublicKeysRevokeResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationPublicKeysRevokeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationPublicKeysRevokeResponseBuilder::success)
    pub fn build(self) -> Result<MutationPublicKeysRevokeResponse, BuildError> {
        Ok(MutationPublicKeysRevokeResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
