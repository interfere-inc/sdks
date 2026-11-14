pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetEnterpriseConnectionActiveWorkspacesRequest {
    #[serde(default)]
    pub args: SetEnterpriseConnectionActiveWorkspacesRequestArgs,
}

impl SetEnterpriseConnectionActiveWorkspacesRequest {
    pub fn builder() -> SetEnterpriseConnectionActiveWorkspacesRequestBuilder {
        <SetEnterpriseConnectionActiveWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetEnterpriseConnectionActiveWorkspacesRequestBuilder {
    args: Option<SetEnterpriseConnectionActiveWorkspacesRequestArgs>,
}

impl SetEnterpriseConnectionActiveWorkspacesRequestBuilder {
    pub fn args(mut self, value: SetEnterpriseConnectionActiveWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetEnterpriseConnectionActiveWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetEnterpriseConnectionActiveWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<SetEnterpriseConnectionActiveWorkspacesRequest, BuildError> {
        Ok(SetEnterpriseConnectionActiveWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

