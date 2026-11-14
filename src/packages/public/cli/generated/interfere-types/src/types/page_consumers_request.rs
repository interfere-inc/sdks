pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageConsumersRequest {
    pub args: PageConsumersRequestArgs,
}

impl PageConsumersRequest {
    pub fn builder() -> PageConsumersRequestBuilder {
        <PageConsumersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageConsumersRequestBuilder {
    args: Option<PageConsumersRequestArgs>,
}

impl PageConsumersRequestBuilder {
    pub fn args(mut self, value: PageConsumersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageConsumersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](PageConsumersRequestBuilder::args)
    pub fn build(self) -> Result<PageConsumersRequest, BuildError> {
        Ok(PageConsumersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

