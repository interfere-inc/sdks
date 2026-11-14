pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetConsumersRequest {
    #[serde(default)]
    pub args: GetConsumersRequestArgs,
}

impl GetConsumersRequest {
    pub fn builder() -> GetConsumersRequestBuilder {
        <GetConsumersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConsumersRequestBuilder {
    args: Option<GetConsumersRequestArgs>,
}

impl GetConsumersRequestBuilder {
    pub fn args(mut self, value: GetConsumersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConsumersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetConsumersRequestBuilder::args)
    pub fn build(self) -> Result<GetConsumersRequest, BuildError> {
        Ok(GetConsumersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

