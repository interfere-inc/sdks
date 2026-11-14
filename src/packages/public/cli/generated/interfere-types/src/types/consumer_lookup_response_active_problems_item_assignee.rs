pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsumerLookupResponseActiveProblemsItemAssignee {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default)]
    pub initials: String,
}

impl ConsumerLookupResponseActiveProblemsItemAssignee {
    pub fn builder() -> ConsumerLookupResponseActiveProblemsItemAssigneeBuilder {
        <ConsumerLookupResponseActiveProblemsItemAssigneeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseActiveProblemsItemAssigneeBuilder {
    id: Option<String>,
    display_name: Option<String>,
    image_url: Option<String>,
    initials: Option<String>,
}

impl ConsumerLookupResponseActiveProblemsItemAssigneeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn initials(mut self, value: impl Into<String>) -> Self {
        self.initials = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseActiveProblemsItemAssignee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConsumerLookupResponseActiveProblemsItemAssigneeBuilder::id)
    /// - [`display_name`](ConsumerLookupResponseActiveProblemsItemAssigneeBuilder::display_name)
    /// - [`initials`](ConsumerLookupResponseActiveProblemsItemAssigneeBuilder::initials)
    pub fn build(self) -> Result<ConsumerLookupResponseActiveProblemsItemAssignee, BuildError> {
        Ok(ConsumerLookupResponseActiveProblemsItemAssignee {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            image_url: self.image_url,
            initials: self.initials.ok_or_else(|| BuildError::missing_field("initials"))?,
        })
    }
}
