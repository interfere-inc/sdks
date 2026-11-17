pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEnvironmentsRequestArgs2 {
    #[serde(default)]
    pub id: String,
}

impl DeleteEnvironmentsRequestArgs2 {
    pub fn builder() -> DeleteEnvironmentsRequestArgs2Builder {
        <DeleteEnvironmentsRequestArgs2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEnvironmentsRequestArgs2Builder {
    id: Option<String>,
}

impl DeleteEnvironmentsRequestArgs2Builder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteEnvironmentsRequestArgs2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteEnvironmentsRequestArgs2Builder::id)
    pub fn build(self) -> Result<DeleteEnvironmentsRequestArgs2, BuildError> {
        Ok(DeleteEnvironmentsRequestArgs2 {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
