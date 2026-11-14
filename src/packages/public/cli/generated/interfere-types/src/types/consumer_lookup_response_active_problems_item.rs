pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsumerLookupResponseActiveProblemsItem {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ConsumerLookupResponseActiveProblemsItemCategory>,
    /// Choose the affected capability: payment for checkout, sign-in for login, search for search, playback for media, layout for misplaced, overlapping or clipped page elements, network for connectivity. A browser exception or failed page render alone does not establish a layout defect. Prefer the specific affected capability over the page containing it. A programming error does not imply bug. Use bug only when the evidence identifies no more specific capability; use unknown when the affected capability is unknown. Never choose by browser, runtime or vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConsumerLookupResponseActiveProblemsItemIcon>,
    pub status: ConsumerLookupResponseActiveProblemsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<ConsumerLookupResponseActiveProblemsItemPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sequence: Option<f64>,
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub affected_users: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub occurrences: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<ConsumerLookupResponseActiveProblemsItemAssignee>,
}

impl ConsumerLookupResponseActiveProblemsItem {
    pub fn builder() -> ConsumerLookupResponseActiveProblemsItemBuilder {
        <ConsumerLookupResponseActiveProblemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseActiveProblemsItemBuilder {
    id: Option<String>,
    title: Option<String>,
    category: Option<ConsumerLookupResponseActiveProblemsItemCategory>,
    icon: Option<ConsumerLookupResponseActiveProblemsItemIcon>,
    status: Option<ConsumerLookupResponseActiveProblemsItemStatus>,
    priority: Option<ConsumerLookupResponseActiveProblemsItemPriority>,
    sequence: Option<f64>,
    affected_users: Option<f64>,
    occurrences: Option<f64>,
    assignee: Option<ConsumerLookupResponseActiveProblemsItemAssignee>,
}

impl ConsumerLookupResponseActiveProblemsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn category(mut self, value: ConsumerLookupResponseActiveProblemsItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn icon(mut self, value: ConsumerLookupResponseActiveProblemsItemIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn status(mut self, value: ConsumerLookupResponseActiveProblemsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: ConsumerLookupResponseActiveProblemsItemPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn sequence(mut self, value: f64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn affected_users(mut self, value: f64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn occurrences(mut self, value: f64) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn assignee(mut self, value: ConsumerLookupResponseActiveProblemsItemAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseActiveProblemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConsumerLookupResponseActiveProblemsItemBuilder::id)
    /// - [`status`](ConsumerLookupResponseActiveProblemsItemBuilder::status)
    pub fn build(self) -> Result<ConsumerLookupResponseActiveProblemsItem, BuildError> {
        Ok(ConsumerLookupResponseActiveProblemsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
            category: self.category,
            icon: self.icon,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            sequence: self.sequence,
            affected_users: self.affected_users,
            occurrences: self.occurrences,
            assignee: self.assignee,
        })
    }
}
