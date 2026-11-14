pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfaceEnvironmentsRequest {
    #[serde(default)]
    pub args: ListForSurfaceEnvironmentsRequestArgs,
}

impl ListForSurfaceEnvironmentsRequest {
    pub fn builder() -> ListForSurfaceEnvironmentsRequestBuilder {
        <ListForSurfaceEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfaceEnvironmentsRequestBuilder {
    args: Option<ListForSurfaceEnvironmentsRequestArgs>,
}

impl ListForSurfaceEnvironmentsRequestBuilder {
    pub fn args(mut self, value: ListForSurfaceEnvironmentsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfaceEnvironmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListForSurfaceEnvironmentsRequestBuilder::args)
    pub fn build(self) -> Result<ListForSurfaceEnvironmentsRequest, BuildError> {
        Ok(ListForSurfaceEnvironmentsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

