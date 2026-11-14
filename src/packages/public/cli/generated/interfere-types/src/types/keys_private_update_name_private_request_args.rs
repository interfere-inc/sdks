pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNamePrivateRequestArgs {
    #[serde(default)]
    pub id: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
}

impl UpdateNamePrivateRequestArgs {
    pub fn builder() -> UpdateNamePrivateRequestArgsBuilder {
        <UpdateNamePrivateRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNamePrivateRequestArgsBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl UpdateNamePrivateRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateNamePrivateRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateNamePrivateRequestArgsBuilder::id)
    /// - [`name`](UpdateNamePrivateRequestArgsBuilder::name)
    pub fn build(self) -> Result<UpdateNamePrivateRequestArgs, BuildError> {
        Ok(UpdateNamePrivateRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
