pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AcceptRequestsRequestArgs {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    pub role: AcceptRequestsRequestArgsRole,
}

impl AcceptRequestsRequestArgs {
    pub fn builder() -> AcceptRequestsRequestArgsBuilder {
        <AcceptRequestsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptRequestsRequestArgsBuilder {
    request_id: Option<String>,
    role: Option<AcceptRequestsRequestArgsRole>,
}

impl AcceptRequestsRequestArgsBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: AcceptRequestsRequestArgsRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AcceptRequestsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](AcceptRequestsRequestArgsBuilder::request_id)
    /// - [`role`](AcceptRequestsRequestArgsBuilder::role)
    pub fn build(self) -> Result<AcceptRequestsRequestArgs, BuildError> {
        Ok(AcceptRequestsRequestArgs {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
