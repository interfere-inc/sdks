pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationEnvironmentsSetReportableResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationEnvironmentsSetReportableResponse {
    pub fn builder() -> MutationEnvironmentsSetReportableResponseBuilder {
        <MutationEnvironmentsSetReportableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationEnvironmentsSetReportableResponseBuilder {
    success: Option<bool>,
}

impl MutationEnvironmentsSetReportableResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationEnvironmentsSetReportableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationEnvironmentsSetReportableResponseBuilder::success)
    pub fn build(self) -> Result<MutationEnvironmentsSetReportableResponse, BuildError> {
        Ok(MutationEnvironmentsSetReportableResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
