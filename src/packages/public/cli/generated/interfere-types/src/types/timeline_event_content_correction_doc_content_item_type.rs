pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentCorrectionDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentCorrectionDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentCorrectionDocContentItemTypeType,
}

impl TimelineEventContentCorrectionDocContentItemType {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemTypeBuilder {
        <TimelineEventContentCorrectionDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentCorrectionDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentCorrectionDocContentItemTypeType>,
}

impl TimelineEventContentCorrectionDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentCorrectionDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentCorrectionDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemType, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
