pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryNetworkAccessAsnResponse {
    #[serde(default)]
    pub asn: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(default)]
    pub generation: i64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
}

impl QueryNetworkAccessAsnResponse {
    pub fn builder() -> QueryNetworkAccessAsnResponseBuilder {
        <QueryNetworkAccessAsnResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNetworkAccessAsnResponseBuilder {
    asn: Option<i64>,
    name: Option<String>,
    organization_name: Option<String>,
    generation: Option<i64>,
    deleted_at: Option<f64>,
}

impl QueryNetworkAccessAsnResponseBuilder {
    pub fn asn(mut self, value: i64) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_name(mut self, value: impl Into<String>) -> Self {
        self.organization_name = Some(value.into());
        self
    }

    pub fn generation(mut self, value: i64) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryNetworkAccessAsnResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asn`](QueryNetworkAccessAsnResponseBuilder::asn)
    /// - [`generation`](QueryNetworkAccessAsnResponseBuilder::generation)
    pub fn build(self) -> Result<QueryNetworkAccessAsnResponse, BuildError> {
        Ok(QueryNetworkAccessAsnResponse {
            asn: self.asn.ok_or_else(|| BuildError::missing_field("asn"))?,
            name: self.name,
            organization_name: self.organization_name,
            generation: self.generation.ok_or_else(|| BuildError::missing_field("generation"))?,
            deleted_at: self.deleted_at,
        })
    }
}
