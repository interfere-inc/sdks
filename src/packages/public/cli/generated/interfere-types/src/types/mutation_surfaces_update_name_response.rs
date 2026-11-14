pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesUpdateNameResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesUpdateNameResponse {
    pub fn builder() -> MutationSurfacesUpdateNameResponseBuilder {
        <MutationSurfacesUpdateNameResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesUpdateNameResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesUpdateNameResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesUpdateNameResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesUpdateNameResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesUpdateNameResponse, BuildError> {
        Ok(MutationSurfacesUpdateNameResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
