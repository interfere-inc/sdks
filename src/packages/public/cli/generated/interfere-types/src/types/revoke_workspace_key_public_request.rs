pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeWorkspaceKeyPublicRequest {
    #[serde(default)]
    pub args: RevokeWorkspaceKeyPublicRequestArgs,
}

impl RevokeWorkspaceKeyPublicRequest {
    pub fn builder() -> RevokeWorkspaceKeyPublicRequestBuilder {
        <RevokeWorkspaceKeyPublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWorkspaceKeyPublicRequestBuilder {
    args: Option<RevokeWorkspaceKeyPublicRequestArgs>,
}

impl RevokeWorkspaceKeyPublicRequestBuilder {
    pub fn args(mut self, value: RevokeWorkspaceKeyPublicRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeWorkspaceKeyPublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RevokeWorkspaceKeyPublicRequestBuilder::args)
    pub fn build(self) -> Result<RevokeWorkspaceKeyPublicRequest, BuildError> {
        Ok(RevokeWorkspaceKeyPublicRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

