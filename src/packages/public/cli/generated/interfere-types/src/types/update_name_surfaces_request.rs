pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNameSurfacesRequest {
    #[serde(default)]
    pub args: UpdateNameSurfacesRequestArgs,
}

impl UpdateNameSurfacesRequest {
    pub fn builder() -> UpdateNameSurfacesRequestBuilder {
        <UpdateNameSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNameSurfacesRequestBuilder {
    args: Option<UpdateNameSurfacesRequestArgs>,
}

impl UpdateNameSurfacesRequestBuilder {
    pub fn args(mut self, value: UpdateNameSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateNameSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateNameSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateNameSurfacesRequest, BuildError> {
        Ok(UpdateNameSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

