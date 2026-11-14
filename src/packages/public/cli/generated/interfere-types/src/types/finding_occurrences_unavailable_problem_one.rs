pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindingOccurrencesUnavailableProblemOne {
    pub code: FindingOccurrencesUnavailableProblemOneCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<FindingOccurrencesUnavailableProblemOneService>,
}

impl FindingOccurrencesUnavailableProblemOne {
    pub fn builder() -> FindingOccurrencesUnavailableProblemOneBuilder {
        <FindingOccurrencesUnavailableProblemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrencesUnavailableProblemOneBuilder {
    code: Option<FindingOccurrencesUnavailableProblemOneCode>,
    message: Option<String>,
    service: Option<FindingOccurrencesUnavailableProblemOneService>,
}

impl FindingOccurrencesUnavailableProblemOneBuilder {
    pub fn code(mut self, value: FindingOccurrencesUnavailableProblemOneCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: FindingOccurrencesUnavailableProblemOneService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FindingOccurrencesUnavailableProblemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](FindingOccurrencesUnavailableProblemOneBuilder::code)
    pub fn build(self) -> Result<FindingOccurrencesUnavailableProblemOne, BuildError> {
        Ok(FindingOccurrencesUnavailableProblemOne {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}
