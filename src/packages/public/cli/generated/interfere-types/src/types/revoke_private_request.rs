pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePrivateRequest {
    #[serde(default)]
    pub args: RevokePrivateRequestArgs,
}

impl RevokePrivateRequest {
    pub fn builder() -> RevokePrivateRequestBuilder {
        <RevokePrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePrivateRequestBuilder {
    args: Option<RevokePrivateRequestArgs>,
}

impl RevokePrivateRequestBuilder {
    pub fn args(mut self, value: RevokePrivateRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RevokePrivateRequestBuilder::args)
    pub fn build(self) -> Result<RevokePrivateRequest, BuildError> {
        Ok(RevokePrivateRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

