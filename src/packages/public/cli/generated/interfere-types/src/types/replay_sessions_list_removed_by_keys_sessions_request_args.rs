pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRemovedByKeysSessionsRequestArgs {
    #[serde(default)]
    pub keys: Vec<String>,
}

impl ListRemovedByKeysSessionsRequestArgs {
    pub fn builder() -> ListRemovedByKeysSessionsRequestArgsBuilder {
        <ListRemovedByKeysSessionsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRemovedByKeysSessionsRequestArgsBuilder {
    keys: Option<Vec<String>>,
}

impl ListRemovedByKeysSessionsRequestArgsBuilder {
    pub fn keys(mut self, value: Vec<String>) -> Self {
        self.keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRemovedByKeysSessionsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keys`](ListRemovedByKeysSessionsRequestArgsBuilder::keys)
    pub fn build(self) -> Result<ListRemovedByKeysSessionsRequestArgs, BuildError> {
        Ok(ListRemovedByKeysSessionsRequestArgs {
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
        })
    }
}
