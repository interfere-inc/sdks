pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdentifySessionProblem {
    pub code: IdentifySessionProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl IdentifySessionProblem {
    pub fn builder() -> IdentifySessionProblemBuilder {
        <IdentifySessionProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentifySessionProblemBuilder {
    code: Option<IdentifySessionProblemCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl IdentifySessionProblemBuilder {
    pub fn code(mut self, value: IdentifySessionProblemCode) -> Self {
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

    /// Consumes the builder and constructs a [`IdentifySessionProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](IdentifySessionProblemBuilder::code)
    pub fn build(self) -> Result<IdentifySessionProblem, BuildError> {
        Ok(IdentifySessionProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
