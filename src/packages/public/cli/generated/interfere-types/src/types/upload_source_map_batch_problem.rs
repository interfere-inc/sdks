pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadSourceMapBatchProblem {
    pub code: UploadSourceMapBatchProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl UploadSourceMapBatchProblem {
    pub fn builder() -> UploadSourceMapBatchProblemBuilder {
        <UploadSourceMapBatchProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadSourceMapBatchProblemBuilder {
    code: Option<UploadSourceMapBatchProblemCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl UploadSourceMapBatchProblemBuilder {
    pub fn code(mut self, value: UploadSourceMapBatchProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: ErrorService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadSourceMapBatchProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UploadSourceMapBatchProblemBuilder::code)
    pub fn build(self) -> Result<UploadSourceMapBatchProblem, BuildError> {
        Ok(UploadSourceMapBatchProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
