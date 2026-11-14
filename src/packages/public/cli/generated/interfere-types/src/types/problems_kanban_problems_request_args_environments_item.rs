pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KanbanProblemsRequestArgsEnvironmentsItem {
    #[serde(rename = "surfaceId")]
    #[serde(default)]
    pub surface_id: String,
    #[serde(default)]
    pub identifiers: Vec<String>,
}

impl KanbanProblemsRequestArgsEnvironmentsItem {
    pub fn builder() -> KanbanProblemsRequestArgsEnvironmentsItemBuilder {
        <KanbanProblemsRequestArgsEnvironmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KanbanProblemsRequestArgsEnvironmentsItemBuilder {
    surface_id: Option<String>,
    identifiers: Option<Vec<String>>,
}

impl KanbanProblemsRequestArgsEnvironmentsItemBuilder {
    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn identifiers(mut self, value: Vec<String>) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KanbanProblemsRequestArgsEnvironmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_id`](KanbanProblemsRequestArgsEnvironmentsItemBuilder::surface_id)
    /// - [`identifiers`](KanbanProblemsRequestArgsEnvironmentsItemBuilder::identifiers)
    pub fn build(self) -> Result<KanbanProblemsRequestArgsEnvironmentsItem, BuildError> {
        Ok(KanbanProblemsRequestArgsEnvironmentsItem {
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
            identifiers: self.identifiers.ok_or_else(|| BuildError::missing_field("identifiers"))?,
        })
    }
}
