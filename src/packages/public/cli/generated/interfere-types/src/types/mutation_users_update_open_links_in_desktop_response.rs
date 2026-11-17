pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationUsersUpdateOpenLinksInDesktopResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationUsersUpdateOpenLinksInDesktopResponse {
    pub fn builder() -> MutationUsersUpdateOpenLinksInDesktopResponseBuilder {
        <MutationUsersUpdateOpenLinksInDesktopResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationUsersUpdateOpenLinksInDesktopResponseBuilder {
    success: Option<bool>,
}

impl MutationUsersUpdateOpenLinksInDesktopResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationUsersUpdateOpenLinksInDesktopResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationUsersUpdateOpenLinksInDesktopResponseBuilder::success)
    pub fn build(self) -> Result<MutationUsersUpdateOpenLinksInDesktopResponse, BuildError> {
        Ok(MutationUsersUpdateOpenLinksInDesktopResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
