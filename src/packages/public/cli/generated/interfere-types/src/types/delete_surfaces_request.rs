pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSurfacesRequest {
    #[serde(default)]
    pub args: DeleteSurfacesRequestArgs,
}

impl DeleteSurfacesRequest {
    pub fn builder() -> DeleteSurfacesRequestBuilder {
        <DeleteSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSurfacesRequestBuilder {
    args: Option<DeleteSurfacesRequestArgs>,
}

impl DeleteSurfacesRequestBuilder {
    pub fn args(mut self, value: DeleteSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DeleteSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<DeleteSurfacesRequest, BuildError> {
        Ok(DeleteSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

