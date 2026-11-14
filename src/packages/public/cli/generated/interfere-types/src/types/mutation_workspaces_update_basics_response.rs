pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesUpdateBasicsResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesUpdateBasicsResponse {
    pub fn builder() -> MutationWorkspacesUpdateBasicsResponseBuilder {
        <MutationWorkspacesUpdateBasicsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesUpdateBasicsResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesUpdateBasicsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesUpdateBasicsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesUpdateBasicsResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesUpdateBasicsResponse, BuildError> {
        Ok(MutationWorkspacesUpdateBasicsResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
