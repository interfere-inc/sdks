pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDefaultSurfaceWorkspacesRequestArgs {
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
}

impl UpdateDefaultSurfaceWorkspacesRequestArgs {
    pub fn builder() -> UpdateDefaultSurfaceWorkspacesRequestArgsBuilder {
        <UpdateDefaultSurfaceWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDefaultSurfaceWorkspacesRequestArgsBuilder {
    surface_id: Option<String>,
}

impl UpdateDefaultSurfaceWorkspacesRequestArgsBuilder {
    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDefaultSurfaceWorkspacesRequestArgs`].
    pub fn build(self) -> Result<UpdateDefaultSurfaceWorkspacesRequestArgs, BuildError> {
        Ok(UpdateDefaultSurfaceWorkspacesRequestArgs {
            surface_id: self.surface_id,
        })
    }
}
