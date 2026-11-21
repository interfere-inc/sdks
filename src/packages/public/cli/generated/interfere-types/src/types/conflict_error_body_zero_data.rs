pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConflictErrorBodyZeroData {
    /// Each kind of record that still refers to the service. Remove these before deleting it.
    #[serde(default)]
    pub blockers: Vec<ConflictErrorBodyZeroDataBlockersItem>,
}

impl ConflictErrorBodyZeroData {
    pub fn builder() -> ConflictErrorBodyZeroDataBuilder {
        <ConflictErrorBodyZeroDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConflictErrorBodyZeroDataBuilder {
    blockers: Option<Vec<ConflictErrorBodyZeroDataBlockersItem>>,
}

impl ConflictErrorBodyZeroDataBuilder {
    pub fn blockers(mut self, value: Vec<ConflictErrorBodyZeroDataBlockersItem>) -> Self {
        self.blockers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConflictErrorBodyZeroData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blockers`](ConflictErrorBodyZeroDataBuilder::blockers)
    pub fn build(self) -> Result<ConflictErrorBodyZeroData, BuildError> {
        Ok(ConflictErrorBodyZeroData {
            blockers: self.blockers.ok_or_else(|| BuildError::missing_field("blockers"))?,
        })
    }
}
