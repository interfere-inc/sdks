pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeSurfacePublicKeyResponse {
    #[serde(default)]
    pub success: bool,
}

impl RevokeSurfacePublicKeyResponse {
    pub fn builder() -> RevokeSurfacePublicKeyResponseBuilder {
        <RevokeSurfacePublicKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeSurfacePublicKeyResponseBuilder {
    success: Option<bool>,
}

impl RevokeSurfacePublicKeyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeSurfacePublicKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RevokeSurfacePublicKeyResponseBuilder::success)
    pub fn build(self) -> Result<RevokeSurfacePublicKeyResponse, BuildError> {
        Ok(RevokeSurfacePublicKeyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
