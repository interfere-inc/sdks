pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentifySessionRequest {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "fpHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp_hash: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "visitorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<IdentifyCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<IdentitySignature>,
    pub source: IdentifySource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<IdentifyTraits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl IdentifySessionRequest {
    pub fn builder() -> IdentifySessionRequestBuilder {
        <IdentifySessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentifySessionRequestBuilder {
    device_id: Option<String>,
    fp_hash: Option<String>,
    session_id: Option<String>,
    visitor_id: Option<String>,
    avatar: Option<String>,
    company: Option<IdentifyCompany>,
    email: Option<String>,
    identifier: Option<String>,
    name: Option<String>,
    signature: Option<IdentitySignature>,
    source: Option<IdentifySource>,
    traits: Option<IdentifyTraits>,
    country: Option<String>,
}

impl IdentifySessionRequestBuilder {
    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn fp_hash(mut self, value: impl Into<String>) -> Self {
        self.fp_hash = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn visitor_id(mut self, value: impl Into<String>) -> Self {
        self.visitor_id = Some(value.into());
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    pub fn company(mut self, value: IdentifyCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn signature(mut self, value: IdentitySignature) -> Self {
        self.signature = Some(value);
        self
    }

    pub fn source(mut self, value: IdentifySource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn traits(mut self, value: IdentifyTraits) -> Self {
        self.traits = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentifySessionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](IdentifySessionRequestBuilder::session_id)
    /// - [`identifier`](IdentifySessionRequestBuilder::identifier)
    /// - [`source`](IdentifySessionRequestBuilder::source)
    pub fn build(self) -> Result<IdentifySessionRequest, BuildError> {
        Ok(IdentifySessionRequest {
            device_id: self.device_id,
            fp_hash: self.fp_hash,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            visitor_id: self.visitor_id,
            avatar: self.avatar,
            company: self.company,
            email: self.email,
            identifier: self.identifier.ok_or_else(|| BuildError::missing_field("identifier"))?,
            name: self.name,
            signature: self.signature,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            traits: self.traits,
            country: self.country,
        })
    }
}

