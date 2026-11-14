pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InspectDomainDnsResponse {
    #[serde(rename = "cnameTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,
    #[serde(rename = "domainExists")]
    #[serde(default)]
    pub domain_exists: bool,
    #[serde(rename = "pointsSomewhere")]
    #[serde(default)]
    pub points_somewhere: bool,
}

impl InspectDomainDnsResponse {
    pub fn builder() -> InspectDomainDnsResponseBuilder {
        <InspectDomainDnsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InspectDomainDnsResponseBuilder {
    cname_target: Option<String>,
    domain_exists: Option<bool>,
    points_somewhere: Option<bool>,
}

impl InspectDomainDnsResponseBuilder {
    pub fn cname_target(mut self, value: impl Into<String>) -> Self {
        self.cname_target = Some(value.into());
        self
    }

    pub fn domain_exists(mut self, value: bool) -> Self {
        self.domain_exists = Some(value);
        self
    }

    pub fn points_somewhere(mut self, value: bool) -> Self {
        self.points_somewhere = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InspectDomainDnsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_exists`](InspectDomainDnsResponseBuilder::domain_exists)
    /// - [`points_somewhere`](InspectDomainDnsResponseBuilder::points_somewhere)
    pub fn build(self) -> Result<InspectDomainDnsResponse, BuildError> {
        Ok(InspectDomainDnsResponse {
            cname_target: self.cname_target,
            domain_exists: self.domain_exists.ok_or_else(|| BuildError::missing_field("domain_exists"))?,
            points_somewhere: self.points_somewhere.ok_or_else(|| BuildError::missing_field("points_somewhere"))?,
        })
    }
}
