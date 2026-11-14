pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompleteSourceMapsProblem {
    pub code: CompleteSourceMapsProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<CompleteSourceMapsProblemService>,
}

impl CompleteSourceMapsProblem {
    pub fn builder() -> CompleteSourceMapsProblemBuilder {
        <CompleteSourceMapsProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteSourceMapsProblemBuilder {
    code: Option<CompleteSourceMapsProblemCode>,
    message: Option<String>,
    service: Option<CompleteSourceMapsProblemService>,
}

impl CompleteSourceMapsProblemBuilder {
    pub fn code(mut self, value: CompleteSourceMapsProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: CompleteSourceMapsProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteSourceMapsProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CompleteSourceMapsProblemBuilder::code)
    pub fn build(self) -> Result<CompleteSourceMapsProblem, BuildError> {
        Ok(CompleteSourceMapsProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
