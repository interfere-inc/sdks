pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetWorkspaceAvatarResponse {
    #[serde(default)]
    pub ok: String,
}

impl SetWorkspaceAvatarResponse {
    pub fn builder() -> SetWorkspaceAvatarResponseBuilder {
        <SetWorkspaceAvatarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetWorkspaceAvatarResponseBuilder {
    ok: Option<String>,
}

impl SetWorkspaceAvatarResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetWorkspaceAvatarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](SetWorkspaceAvatarResponseBuilder::ok)
    pub fn build(self) -> Result<SetWorkspaceAvatarResponse, BuildError> {
        Ok(SetWorkspaceAvatarResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
