pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetEnabledDirectoriesRequestArgs {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub id: String,
}

impl SetEnabledDirectoriesRequestArgs {
    pub fn builder() -> SetEnabledDirectoriesRequestArgsBuilder {
        <SetEnabledDirectoriesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetEnabledDirectoriesRequestArgsBuilder {
    enabled: Option<bool>,
    id: Option<String>,
}

impl SetEnabledDirectoriesRequestArgsBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetEnabledDirectoriesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](SetEnabledDirectoriesRequestArgsBuilder::enabled)
    /// - [`id`](SetEnabledDirectoriesRequestArgsBuilder::id)
    pub fn build(self) -> Result<SetEnabledDirectoriesRequestArgs, BuildError> {
        Ok(SetEnabledDirectoriesRequestArgs {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
