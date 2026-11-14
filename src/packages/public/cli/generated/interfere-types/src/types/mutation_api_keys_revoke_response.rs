pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationApiKeysRevokeResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationApiKeysRevokeResponse {
    pub fn builder() -> MutationApiKeysRevokeResponseBuilder {
        <MutationApiKeysRevokeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationApiKeysRevokeResponseBuilder {
    success: Option<bool>,
}

impl MutationApiKeysRevokeResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationApiKeysRevokeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationApiKeysRevokeResponseBuilder::success)
    pub fn build(self) -> Result<MutationApiKeysRevokeResponse, BuildError> {
        Ok(MutationApiKeysRevokeResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
