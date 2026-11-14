pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIntegrationsRequest {
    #[serde(default)]
    pub args: GetIntegrationsRequestArgs,
}

impl GetIntegrationsRequest {
    pub fn builder() -> GetIntegrationsRequestBuilder {
        <GetIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIntegrationsRequestBuilder {
    args: Option<GetIntegrationsRequestArgs>,
}

impl GetIntegrationsRequestBuilder {
    pub fn args(mut self, value: GetIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<GetIntegrationsRequest, BuildError> {
        Ok(GetIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

