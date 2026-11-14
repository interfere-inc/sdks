pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfacePublicRequest {
    #[serde(default)]
    pub args: ListForSurfacePublicRequestArgs,
}

impl ListForSurfacePublicRequest {
    pub fn builder() -> ListForSurfacePublicRequestBuilder {
        <ListForSurfacePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfacePublicRequestBuilder {
    args: Option<ListForSurfacePublicRequestArgs>,
}

impl ListForSurfacePublicRequestBuilder {
    pub fn args(mut self, value: ListForSurfacePublicRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfacePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListForSurfacePublicRequestBuilder::args)
    pub fn build(self) -> Result<ListForSurfacePublicRequest, BuildError> {
        Ok(ListForSurfacePublicRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

