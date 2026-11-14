pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNamePrivateRequest {
    #[serde(default)]
    pub args: UpdateNamePrivateRequestArgs,
}

impl UpdateNamePrivateRequest {
    pub fn builder() -> UpdateNamePrivateRequestBuilder {
        <UpdateNamePrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNamePrivateRequestBuilder {
    args: Option<UpdateNamePrivateRequestArgs>,
}

impl UpdateNamePrivateRequestBuilder {
    pub fn args(mut self, value: UpdateNamePrivateRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateNamePrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateNamePrivateRequestBuilder::args)
    pub fn build(self) -> Result<UpdateNamePrivateRequest, BuildError> {
        Ok(UpdateNamePrivateRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

