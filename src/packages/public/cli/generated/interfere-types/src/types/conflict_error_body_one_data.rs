pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConflictErrorBodyOneData {
    pub field: ConflictErrorBodyOneDataField,
}

impl ConflictErrorBodyOneData {
    pub fn builder() -> ConflictErrorBodyOneDataBuilder {
        <ConflictErrorBodyOneDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConflictErrorBodyOneDataBuilder {
    field: Option<ConflictErrorBodyOneDataField>,
}

impl ConflictErrorBodyOneDataBuilder {
    pub fn field(mut self, value: ConflictErrorBodyOneDataField) -> Self {
        self.field = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConflictErrorBodyOneData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ConflictErrorBodyOneDataBuilder::field)
    pub fn build(self) -> Result<ConflictErrorBodyOneData, BuildError> {
        Ok(ConflictErrorBodyOneData {
            field: self.field.ok_or_else(|| BuildError::missing_field("field"))?,
        })
    }
}
