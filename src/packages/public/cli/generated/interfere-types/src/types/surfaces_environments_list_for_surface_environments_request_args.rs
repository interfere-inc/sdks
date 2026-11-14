pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfaceEnvironmentsRequestArgs {
    #[serde(rename = "surfaceId")]
    #[serde(default)]
    pub surface_id: String,
}

impl ListForSurfaceEnvironmentsRequestArgs {
    pub fn builder() -> ListForSurfaceEnvironmentsRequestArgsBuilder {
        <ListForSurfaceEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfaceEnvironmentsRequestArgsBuilder {
    surface_id: Option<String>,
}

impl ListForSurfaceEnvironmentsRequestArgsBuilder {
    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfaceEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_id`](ListForSurfaceEnvironmentsRequestArgsBuilder::surface_id)
    pub fn build(self) -> Result<ListForSurfaceEnvironmentsRequestArgs, BuildError> {
        Ok(ListForSurfaceEnvironmentsRequestArgs {
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
        })
    }
}
