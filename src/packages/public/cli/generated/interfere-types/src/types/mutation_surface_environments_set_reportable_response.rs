pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfaceEnvironmentsSetReportableResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfaceEnvironmentsSetReportableResponse {
    pub fn builder() -> MutationSurfaceEnvironmentsSetReportableResponseBuilder {
        <MutationSurfaceEnvironmentsSetReportableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfaceEnvironmentsSetReportableResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfaceEnvironmentsSetReportableResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfaceEnvironmentsSetReportableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfaceEnvironmentsSetReportableResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfaceEnvironmentsSetReportableResponse, BuildError> {
        Ok(MutationSurfaceEnvironmentsSetReportableResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
