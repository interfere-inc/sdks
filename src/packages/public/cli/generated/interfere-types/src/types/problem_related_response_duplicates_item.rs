pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponseDuplicatesItem {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    pub status: ProblemRelatedResponseDuplicatesItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ProblemRelatedResponseDuplicatesItem {
    pub fn builder() -> ProblemRelatedResponseDuplicatesItemBuilder {
        <ProblemRelatedResponseDuplicatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponseDuplicatesItemBuilder {
    id: Option<String>,
    sequence: Option<i64>,
    status: Option<ProblemRelatedResponseDuplicatesItemStatus>,
    title: Option<String>,
}

impl ProblemRelatedResponseDuplicatesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn status(mut self, value: ProblemRelatedResponseDuplicatesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponseDuplicatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProblemRelatedResponseDuplicatesItemBuilder::id)
    /// - [`status`](ProblemRelatedResponseDuplicatesItemBuilder::status)
    pub fn build(self) -> Result<ProblemRelatedResponseDuplicatesItem, BuildError> {
        Ok(ProblemRelatedResponseDuplicatesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            sequence: self.sequence,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
        })
    }
}
