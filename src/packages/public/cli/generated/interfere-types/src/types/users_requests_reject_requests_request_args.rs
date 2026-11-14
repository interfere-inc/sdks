pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectRequestsRequestArgs {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
}

impl RejectRequestsRequestArgs {
    pub fn builder() -> RejectRequestsRequestArgsBuilder {
        <RejectRequestsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectRequestsRequestArgsBuilder {
    request_id: Option<String>,
}

impl RejectRequestsRequestArgsBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RejectRequestsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](RejectRequestsRequestArgsBuilder::request_id)
    pub fn build(self) -> Result<RejectRequestsRequestArgs, BuildError> {
        Ok(RejectRequestsRequestArgs {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
        })
    }
}
