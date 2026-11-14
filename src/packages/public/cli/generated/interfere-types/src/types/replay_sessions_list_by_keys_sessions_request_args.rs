pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListByKeysSessionsRequestArgs {
    #[serde(default)]
    pub keys: Vec<String>,
}

impl ListByKeysSessionsRequestArgs {
    pub fn builder() -> ListByKeysSessionsRequestArgsBuilder {
        <ListByKeysSessionsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListByKeysSessionsRequestArgsBuilder {
    keys: Option<Vec<String>>,
}

impl ListByKeysSessionsRequestArgsBuilder {
    pub fn keys(mut self, value: Vec<String>) -> Self {
        self.keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListByKeysSessionsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keys`](ListByKeysSessionsRequestArgsBuilder::keys)
    pub fn build(self) -> Result<ListByKeysSessionsRequestArgs, BuildError> {
        Ok(ListByKeysSessionsRequestArgs {
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
        })
    }
}
