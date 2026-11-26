pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrectFixRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<CorrectFixRequestLocationsItem>>,
    pub reason: CorrectFixRequestReason,
}

impl CorrectFixRequest {
    pub fn builder() -> CorrectFixRequestBuilder {
        <CorrectFixRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectFixRequestBuilder {
    request_id: Option<String>,
    summary: Option<String>,
    locations: Option<Vec<CorrectFixRequestLocationsItem>>,
    reason: Option<CorrectFixRequestReason>,
}

impl CorrectFixRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn locations(mut self, value: Vec<CorrectFixRequestLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn reason(mut self, value: CorrectFixRequestReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectFixRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](CorrectFixRequestBuilder::request_id)
    /// - [`summary`](CorrectFixRequestBuilder::summary)
    /// - [`reason`](CorrectFixRequestBuilder::reason)
    pub fn build(self) -> Result<CorrectFixRequest, BuildError> {
        Ok(CorrectFixRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            locations: self.locations,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}

