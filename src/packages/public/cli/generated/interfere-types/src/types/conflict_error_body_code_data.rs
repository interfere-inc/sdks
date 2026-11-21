pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConflictErrorBodyCodeData {
    /// Resources of the services sharing this namespace and name, null for the one without a resource, at most 50.
    #[serde(default)]
    pub resources: Vec<Option<String>>,
    /// True when more services share the name than resources lists.
    #[serde(default)]
    pub truncated: bool,
}

impl ConflictErrorBodyCodeData {
    pub fn builder() -> ConflictErrorBodyCodeDataBuilder {
        <ConflictErrorBodyCodeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConflictErrorBodyCodeDataBuilder {
    resources: Option<Vec<Option<String>>>,
    truncated: Option<bool>,
}

impl ConflictErrorBodyCodeDataBuilder {
    pub fn resources(mut self, value: Vec<Option<String>>) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConflictErrorBodyCodeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resources`](ConflictErrorBodyCodeDataBuilder::resources)
    /// - [`truncated`](ConflictErrorBodyCodeDataBuilder::truncated)
    pub fn build(self) -> Result<ConflictErrorBodyCodeData, BuildError> {
        Ok(ConflictErrorBodyCodeData {
            resources: self.resources.ok_or_else(|| BuildError::missing_field("resources"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
