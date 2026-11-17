pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByNameEnvironmentsRequestArgs {
    /// Workspace environment name as reported by the SDK, lowercased, such as production.
    #[serde(default)]
    pub name: String,
}

impl ByNameEnvironmentsRequestArgs {
    pub fn builder() -> ByNameEnvironmentsRequestArgsBuilder {
        <ByNameEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByNameEnvironmentsRequestArgsBuilder {
    name: Option<String>,
}

impl ByNameEnvironmentsRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ByNameEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ByNameEnvironmentsRequestArgsBuilder::name)
    pub fn build(self) -> Result<ByNameEnvironmentsRequestArgs, BuildError> {
        Ok(ByNameEnvironmentsRequestArgs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
