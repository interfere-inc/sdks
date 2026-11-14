pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainsRequestArgs {
    #[serde(rename = "domainId")]
    #[serde(default)]
    pub domain_id: String,
}

impl GetDomainsRequestArgs {
    pub fn builder() -> GetDomainsRequestArgsBuilder {
        <GetDomainsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainsRequestArgsBuilder {
    domain_id: Option<String>,
}

impl GetDomainsRequestArgsBuilder {
    pub fn domain_id(mut self, value: impl Into<String>) -> Self {
        self.domain_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDomainsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_id`](GetDomainsRequestArgsBuilder::domain_id)
    pub fn build(self) -> Result<GetDomainsRequestArgs, BuildError> {
        Ok(GetDomainsRequestArgs {
            domain_id: self.domain_id.ok_or_else(|| BuildError::missing_field("domain_id"))?,
        })
    }
}
