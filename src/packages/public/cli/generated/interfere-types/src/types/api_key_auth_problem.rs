pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKeyAuthProblem {
    pub code: ApiKeyAuthProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl ApiKeyAuthProblem {
    pub fn builder() -> ApiKeyAuthProblemBuilder {
        <ApiKeyAuthProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyAuthProblemBuilder {
    code: Option<ApiKeyAuthProblemCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl ApiKeyAuthProblemBuilder {
    pub fn code(mut self, value: ApiKeyAuthProblemCode) -> Self {
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

    /// Consumes the builder and constructs a [`ApiKeyAuthProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ApiKeyAuthProblemBuilder::code)
    pub fn build(self) -> Result<ApiKeyAuthProblem, BuildError> {
        Ok(ApiKeyAuthProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
