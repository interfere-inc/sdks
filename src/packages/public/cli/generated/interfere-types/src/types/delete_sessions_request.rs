pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSessionsRequest {
    #[serde(default)]
    pub args: DeleteSessionsRequestArgs,
}

impl DeleteSessionsRequest {
    pub fn builder() -> DeleteSessionsRequestBuilder {
        <DeleteSessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSessionsRequestBuilder {
    args: Option<DeleteSessionsRequestArgs>,
}

impl DeleteSessionsRequestBuilder {
    pub fn args(mut self, value: DeleteSessionsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteSessionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DeleteSessionsRequestBuilder::args)
    pub fn build(self) -> Result<DeleteSessionsRequest, BuildError> {
        Ok(DeleteSessionsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

