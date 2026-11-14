pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LookupNetworkAsnRequest {
    #[serde(default)]
    pub asn: i64,
}

impl LookupNetworkAsnRequest {
    pub fn builder() -> LookupNetworkAsnRequestBuilder {
        <LookupNetworkAsnRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LookupNetworkAsnRequestBuilder {
    asn: Option<i64>,
}

impl LookupNetworkAsnRequestBuilder {
    pub fn asn(mut self, value: i64) -> Self {
        self.asn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LookupNetworkAsnRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asn`](LookupNetworkAsnRequestBuilder::asn)
    pub fn build(self) -> Result<LookupNetworkAsnRequest, BuildError> {
        Ok(LookupNetworkAsnRequest {
            asn: self.asn.ok_or_else(|| BuildError::missing_field("asn"))?,
        })
    }
}

