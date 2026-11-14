pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfacePrivateRequest {
    #[serde(default)]
    pub args: ListForSurfacePrivateRequestArgs,
}

impl ListForSurfacePrivateRequest {
    pub fn builder() -> ListForSurfacePrivateRequestBuilder {
        <ListForSurfacePrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfacePrivateRequestBuilder {
    args: Option<ListForSurfacePrivateRequestArgs>,
}

impl ListForSurfacePrivateRequestBuilder {
    pub fn args(mut self, value: ListForSurfacePrivateRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfacePrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListForSurfacePrivateRequestBuilder::args)
    pub fn build(self) -> Result<ListForSurfacePrivateRequest, BuildError> {
        Ok(ListForSurfacePrivateRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

