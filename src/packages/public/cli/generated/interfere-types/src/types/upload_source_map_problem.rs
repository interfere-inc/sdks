pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadSourceMapProblem {
    pub code: UploadSourceMapProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<UploadSourceMapProblemService>,
}

impl UploadSourceMapProblem {
    pub fn builder() -> UploadSourceMapProblemBuilder {
        <UploadSourceMapProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadSourceMapProblemBuilder {
    code: Option<UploadSourceMapProblemCode>,
    message: Option<String>,
    service: Option<UploadSourceMapProblemService>,
}

impl UploadSourceMapProblemBuilder {
    pub fn code(mut self, value: UploadSourceMapProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: UploadSourceMapProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadSourceMapProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UploadSourceMapProblemBuilder::code)
    pub fn build(self) -> Result<UploadSourceMapProblem, BuildError> {
        Ok(UploadSourceMapProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
