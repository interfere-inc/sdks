pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SourceMapUploadSessionResponse {
    #[serde(rename = "batchUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_upload: Option<SourceMapBatchCapability>,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: i64,
    /// Opaque grant for this upload session. Send it as the X-Interfere-Upload-Grant header on chunk and batch uploads in place of the API key; it expires with the session.
    #[serde(rename = "uploadGrant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_grant: Option<String>,
    #[serde(default)]
    pub uploads: Vec<SourceMapUploadSessionResponseUploadsItem>,
}

impl SourceMapUploadSessionResponse {
    pub fn builder() -> SourceMapUploadSessionResponseBuilder {
        <SourceMapUploadSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapUploadSessionResponseBuilder {
    batch_upload: Option<SourceMapBatchCapability>,
    expires_at: Option<i64>,
    upload_grant: Option<String>,
    uploads: Option<Vec<SourceMapUploadSessionResponseUploadsItem>>,
}

impl SourceMapUploadSessionResponseBuilder {
    pub fn batch_upload(mut self, value: SourceMapBatchCapability) -> Self {
        self.batch_upload = Some(value);
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn upload_grant(mut self, value: impl Into<String>) -> Self {
        self.upload_grant = Some(value.into());
        self
    }

    pub fn uploads(mut self, value: Vec<SourceMapUploadSessionResponseUploadsItem>) -> Self {
        self.uploads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapUploadSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](SourceMapUploadSessionResponseBuilder::expires_at)
    /// - [`uploads`](SourceMapUploadSessionResponseBuilder::uploads)
    pub fn build(self) -> Result<SourceMapUploadSessionResponse, BuildError> {
        Ok(SourceMapUploadSessionResponse {
            batch_upload: self.batch_upload,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            upload_grant: self.upload_grant,
            uploads: self.uploads.ok_or_else(|| BuildError::missing_field("uploads"))?,
        })
    }
}
