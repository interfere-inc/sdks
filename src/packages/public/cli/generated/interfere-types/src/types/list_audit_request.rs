pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAuditRequest {
    #[serde(default)]
    pub args: ListAuditRequestArgs,
}

impl ListAuditRequest {
    pub fn builder() -> ListAuditRequestBuilder {
        <ListAuditRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditRequestBuilder {
    args: Option<ListAuditRequestArgs>,
}

impl ListAuditRequestBuilder {
    pub fn args(mut self, value: ListAuditRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuditRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListAuditRequestBuilder::args)
    pub fn build(self) -> Result<ListAuditRequest, BuildError> {
        Ok(ListAuditRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

