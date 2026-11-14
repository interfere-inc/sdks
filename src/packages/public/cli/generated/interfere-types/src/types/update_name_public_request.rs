pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNamePublicRequest {
    #[serde(default)]
    pub args: UpdateNamePublicRequestArgs,
}

impl UpdateNamePublicRequest {
    pub fn builder() -> UpdateNamePublicRequestBuilder {
        <UpdateNamePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNamePublicRequestBuilder {
    args: Option<UpdateNamePublicRequestArgs>,
}

impl UpdateNamePublicRequestBuilder {
    pub fn args(mut self, value: UpdateNamePublicRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateNamePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateNamePublicRequestBuilder::args)
    pub fn build(self) -> Result<UpdateNamePublicRequest, BuildError> {
        Ok(UpdateNamePublicRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

