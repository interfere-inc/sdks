pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAuditRequestArgs {
    #[serde(default)]
    pub id: String,
}

impl GetAuditRequestArgs {
    pub fn builder() -> GetAuditRequestArgsBuilder {
        <GetAuditRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAuditRequestArgsBuilder {
    id: Option<String>,
}

impl GetAuditRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAuditRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetAuditRequestArgsBuilder::id)
    pub fn build(self) -> Result<GetAuditRequestArgs, BuildError> {
        Ok(GetAuditRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
