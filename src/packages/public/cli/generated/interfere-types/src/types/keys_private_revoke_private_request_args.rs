pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePrivateRequestArgs {
    #[serde(default)]
    pub id: String,
}

impl RevokePrivateRequestArgs {
    pub fn builder() -> RevokePrivateRequestArgsBuilder {
        <RevokePrivateRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePrivateRequestArgsBuilder {
    id: Option<String>,
}

impl RevokePrivateRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokePrivateRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokePrivateRequestArgsBuilder::id)
    pub fn build(self) -> Result<RevokePrivateRequestArgs, BuildError> {
        Ok(RevokePrivateRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
