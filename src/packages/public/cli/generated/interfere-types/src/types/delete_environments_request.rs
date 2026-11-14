pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEnvironmentsRequest {
    #[serde(default)]
    pub args: DeleteEnvironmentsRequestArgs,
}

impl DeleteEnvironmentsRequest {
    pub fn builder() -> DeleteEnvironmentsRequestBuilder {
        <DeleteEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEnvironmentsRequestBuilder {
    args: Option<DeleteEnvironmentsRequestArgs>,
}

impl DeleteEnvironmentsRequestBuilder {
    pub fn args(mut self, value: DeleteEnvironmentsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteEnvironmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DeleteEnvironmentsRequestBuilder::args)
    pub fn build(self) -> Result<DeleteEnvironmentsRequest, BuildError> {
        Ok(DeleteEnvironmentsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

