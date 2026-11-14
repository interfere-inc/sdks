pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IngestUnavailableProblem {
    pub code: IngestUnavailableProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<IngestUnavailableProblemService>,
}

impl IngestUnavailableProblem {
    pub fn builder() -> IngestUnavailableProblemBuilder {
        <IngestUnavailableProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestUnavailableProblemBuilder {
    code: Option<IngestUnavailableProblemCode>,
    message: Option<String>,
    service: Option<IngestUnavailableProblemService>,
}

impl IngestUnavailableProblemBuilder {
    pub fn code(mut self, value: IngestUnavailableProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: IngestUnavailableProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestUnavailableProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](IngestUnavailableProblemBuilder::code)
    pub fn build(self) -> Result<IngestUnavailableProblem, BuildError> {
        Ok(IngestUnavailableProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
