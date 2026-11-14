pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionBatchEventsResponseSessionsValueItemProblem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<SessionBatchEventsResponseSessionsValueItemProblemCategory>,
    /// Choose the affected capability: payment for checkout, sign-in for login, search for search, playback for media, layout for misplaced, overlapping or clipped page elements, network for connectivity. A browser exception or failed page render alone does not establish a layout defect. Prefer the specific affected capability over the page containing it. A programming error does not imply bug. Use bug only when the evidence identifies no more specific capability; use unknown when the affected capability is unknown. Never choose by browser, runtime or vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<SessionBatchEventsResponseSessionsValueItemProblemIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sequence: Option<f64>,
}

impl SessionBatchEventsResponseSessionsValueItemProblem {
    pub fn builder() -> SessionBatchEventsResponseSessionsValueItemProblemBuilder {
        <SessionBatchEventsResponseSessionsValueItemProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionBatchEventsResponseSessionsValueItemProblemBuilder {
    id: Option<String>,
    fingerprint: Option<String>,
    title: Option<String>,
    category: Option<SessionBatchEventsResponseSessionsValueItemProblemCategory>,
    icon: Option<SessionBatchEventsResponseSessionsValueItemProblemIcon>,
    sequence: Option<f64>,
}

impl SessionBatchEventsResponseSessionsValueItemProblemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn category(mut self, value: SessionBatchEventsResponseSessionsValueItemProblemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn icon(mut self, value: SessionBatchEventsResponseSessionsValueItemProblemIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn sequence(mut self, value: f64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionBatchEventsResponseSessionsValueItemProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SessionBatchEventsResponseSessionsValueItemProblemBuilder::id)
    /// - [`fingerprint`](SessionBatchEventsResponseSessionsValueItemProblemBuilder::fingerprint)
    pub fn build(self) -> Result<SessionBatchEventsResponseSessionsValueItemProblem, BuildError> {
        Ok(SessionBatchEventsResponseSessionsValueItemProblem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            title: self.title,
            category: self.category,
            icon: self.icon,
            sequence: self.sequence,
        })
    }
}
