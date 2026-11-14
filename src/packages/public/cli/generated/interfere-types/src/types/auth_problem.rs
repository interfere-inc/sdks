pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthProblem {
    pub code: AuthProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<AuthProblemService>,
}

impl AuthProblem {
    pub fn builder() -> AuthProblemBuilder {
        <AuthProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthProblemBuilder {
    code: Option<AuthProblemCode>,
    message: Option<String>,
    service: Option<AuthProblemService>,
}

impl AuthProblemBuilder {
    pub fn code(mut self, value: AuthProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: AuthProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](AuthProblemBuilder::code)
    pub fn build(self) -> Result<AuthProblem, BuildError> {
        Ok(AuthProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
