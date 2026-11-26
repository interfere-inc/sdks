pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForWorkspaceServicesRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ForWorkspaceServicesRequestArgs {
    pub fn builder() -> ForWorkspaceServicesRequestArgsBuilder {
        <ForWorkspaceServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForWorkspaceServicesRequestArgsBuilder {
    limit: Option<i64>,
}

impl ForWorkspaceServicesRequestArgsBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForWorkspaceServicesRequestArgs`].
    pub fn build(self) -> Result<ForWorkspaceServicesRequestArgs, BuildError> {
        Ok(ForWorkspaceServicesRequestArgs {
            limit: self.limit,
        })
    }
}
