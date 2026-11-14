pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnNetworkRequestArgs {
    #[serde(default)]
    pub asn: i64,
}

impl AsnNetworkRequestArgs {
    pub fn builder() -> AsnNetworkRequestArgsBuilder {
        <AsnNetworkRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnNetworkRequestArgsBuilder {
    asn: Option<i64>,
}

impl AsnNetworkRequestArgsBuilder {
    pub fn asn(mut self, value: i64) -> Self {
        self.asn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnNetworkRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asn`](AsnNetworkRequestArgsBuilder::asn)
    pub fn build(self) -> Result<AsnNetworkRequestArgs, BuildError> {
        Ok(AsnNetworkRequestArgs {
            asn: self.asn.ok_or_else(|| BuildError::missing_field("asn"))?,
        })
    }
}
