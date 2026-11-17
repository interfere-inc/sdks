pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOpenLinksInDesktopUsersRequest {
    #[serde(default)]
    pub args: UpdateOpenLinksInDesktopUsersRequestArgs,
}

impl UpdateOpenLinksInDesktopUsersRequest {
    pub fn builder() -> UpdateOpenLinksInDesktopUsersRequestBuilder {
        <UpdateOpenLinksInDesktopUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOpenLinksInDesktopUsersRequestBuilder {
    args: Option<UpdateOpenLinksInDesktopUsersRequestArgs>,
}

impl UpdateOpenLinksInDesktopUsersRequestBuilder {
    pub fn args(mut self, value: UpdateOpenLinksInDesktopUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOpenLinksInDesktopUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateOpenLinksInDesktopUsersRequestBuilder::args)
    pub fn build(self) -> Result<UpdateOpenLinksInDesktopUsersRequest, BuildError> {
        Ok(UpdateOpenLinksInDesktopUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

