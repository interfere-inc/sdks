pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEnvironmentsRequestArgs {
    /// Workspace environment name as reported by the SDK, lowercased, such as production.
    #[serde(default)]
    pub name: String,
}

impl DeleteEnvironmentsRequestArgs {
    pub fn builder() -> DeleteEnvironmentsRequestArgsBuilder {
        <DeleteEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEnvironmentsRequestArgsBuilder {
    name: Option<String>,
}

impl DeleteEnvironmentsRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeleteEnvironmentsRequestArgsBuilder::name)
    pub fn build(self) -> Result<DeleteEnvironmentsRequestArgs, BuildError> {
        Ok(DeleteEnvironmentsRequestArgs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
