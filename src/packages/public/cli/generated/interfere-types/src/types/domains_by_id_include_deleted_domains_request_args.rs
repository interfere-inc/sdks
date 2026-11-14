pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByIdIncludeDeletedDomainsRequestArgs {
    #[serde(rename = "domainId")]
    #[serde(default)]
    pub domain_id: String,
}

impl ByIdIncludeDeletedDomainsRequestArgs {
    pub fn builder() -> ByIdIncludeDeletedDomainsRequestArgsBuilder {
        <ByIdIncludeDeletedDomainsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByIdIncludeDeletedDomainsRequestArgsBuilder {
    domain_id: Option<String>,
}

impl ByIdIncludeDeletedDomainsRequestArgsBuilder {
    pub fn domain_id(mut self, value: impl Into<String>) -> Self {
        self.domain_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ByIdIncludeDeletedDomainsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_id`](ByIdIncludeDeletedDomainsRequestArgsBuilder::domain_id)
    pub fn build(self) -> Result<ByIdIncludeDeletedDomainsRequestArgs, BuildError> {
        Ok(ByIdIncludeDeletedDomainsRequestArgs {
            domain_id: self.domain_id.ok_or_else(|| BuildError::missing_field("domain_id"))?,
        })
    }
}
