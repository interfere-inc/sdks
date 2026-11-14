pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEnvironmentsRequestArgs {
    #[serde(default)]
    pub id: String,
}

impl DeleteEnvironmentsRequestArgs {
    pub fn builder() -> DeleteEnvironmentsRequestArgsBuilder {
        <DeleteEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEnvironmentsRequestArgsBuilder {
    id: Option<String>,
}

impl DeleteEnvironmentsRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteEnvironmentsRequestArgsBuilder::id)
    pub fn build(self) -> Result<DeleteEnvironmentsRequestArgs, BuildError> {
        Ok(DeleteEnvironmentsRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
