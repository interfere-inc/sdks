pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitWorkspaceFeedbackRequestAttachmentsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub file: String,
}

impl SubmitWorkspaceFeedbackRequestAttachmentsItem {
    pub fn builder() -> SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder {
        <SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder {
    id: Option<String>,
    file: Option<String>,
}

impl SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitWorkspaceFeedbackRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder::id)
    /// - [`file`](SubmitWorkspaceFeedbackRequestAttachmentsItemBuilder::file)
    pub fn build(self) -> Result<SubmitWorkspaceFeedbackRequestAttachmentsItem, BuildError> {
        Ok(SubmitWorkspaceFeedbackRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
