pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnNetworkRequest {
    #[serde(default)]
    pub args: AsnNetworkRequestArgs,
}

impl AsnNetworkRequest {
    pub fn builder() -> AsnNetworkRequestBuilder {
        <AsnNetworkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnNetworkRequestBuilder {
    args: Option<AsnNetworkRequestArgs>,
}

impl AsnNetworkRequestBuilder {
    pub fn args(mut self, value: AsnNetworkRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnNetworkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](AsnNetworkRequestBuilder::args)
    pub fn build(self) -> Result<AsnNetworkRequest, BuildError> {
        Ok(AsnNetworkRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

