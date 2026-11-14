pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NetworkAccessResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "organizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(default)]
    pub allowed: bool,
}

impl NetworkAccessResponse {
    pub fn builder() -> NetworkAccessResponseBuilder {
        <NetworkAccessResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NetworkAccessResponseBuilder {
    ip: Option<String>,
    asn: Option<i64>,
    organization_name: Option<String>,
    allowed: Option<bool>,
}

impl NetworkAccessResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn asn(mut self, value: i64) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn organization_name(mut self, value: impl Into<String>) -> Self {
        self.organization_name = Some(value.into());
        self
    }

    pub fn allowed(mut self, value: bool) -> Self {
        self.allowed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NetworkAccessResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed`](NetworkAccessResponseBuilder::allowed)
    pub fn build(self) -> Result<NetworkAccessResponse, BuildError> {
        Ok(NetworkAccessResponse {
            ip: self.ip,
            asn: self.asn,
            organization_name: self.organization_name,
            allowed: self.allowed.ok_or_else(|| BuildError::missing_field("allowed"))?,
        })
    }
}
