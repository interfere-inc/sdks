pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAuditRequest {
    #[serde(default)]
    pub args: GetAuditRequestArgs,
}

impl GetAuditRequest {
    pub fn builder() -> GetAuditRequestBuilder {
        <GetAuditRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAuditRequestBuilder {
    args: Option<GetAuditRequestArgs>,
}

impl GetAuditRequestBuilder {
    pub fn args(mut self, value: GetAuditRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAuditRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetAuditRequestBuilder::args)
    pub fn build(self) -> Result<GetAuditRequest, BuildError> {
        Ok(GetAuditRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

