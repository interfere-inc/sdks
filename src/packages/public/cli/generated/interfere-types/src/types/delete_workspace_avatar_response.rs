pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteWorkspaceAvatarResponse {
    #[serde(default)]
    pub ok: String,
}

impl DeleteWorkspaceAvatarResponse {
    pub fn builder() -> DeleteWorkspaceAvatarResponseBuilder {
        <DeleteWorkspaceAvatarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteWorkspaceAvatarResponseBuilder {
    ok: Option<String>,
}

impl DeleteWorkspaceAvatarResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteWorkspaceAvatarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](DeleteWorkspaceAvatarResponseBuilder::ok)
    pub fn build(self) -> Result<DeleteWorkspaceAvatarResponse, BuildError> {
        Ok(DeleteWorkspaceAvatarResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
