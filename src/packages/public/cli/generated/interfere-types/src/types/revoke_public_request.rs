pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePublicRequest {
    #[serde(default)]
    pub args: RevokePublicRequestArgs,
}

impl RevokePublicRequest {
    pub fn builder() -> RevokePublicRequestBuilder {
        <RevokePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePublicRequestBuilder {
    args: Option<RevokePublicRequestArgs>,
}

impl RevokePublicRequestBuilder {
    pub fn args(mut self, value: RevokePublicRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RevokePublicRequestBuilder::args)
    pub fn build(self) -> Result<RevokePublicRequest, BuildError> {
        Ok(RevokePublicRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

