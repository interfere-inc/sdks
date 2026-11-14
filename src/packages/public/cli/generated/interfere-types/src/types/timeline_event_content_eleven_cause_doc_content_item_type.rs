pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentElevenCauseDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentElevenCauseDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentElevenCauseDocContentItemTypeType,
}

impl TimelineEventContentElevenCauseDocContentItemType {
    pub fn builder() -> TimelineEventContentElevenCauseDocContentItemTypeBuilder {
        <TimelineEventContentElevenCauseDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentElevenCauseDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentElevenCauseDocContentItemTypeType>,
}

impl TimelineEventContentElevenCauseDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentElevenCauseDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenCauseDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCauseDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentElevenCauseDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentElevenCauseDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenCauseDocContentItemType, BuildError> {
        Ok(TimelineEventContentElevenCauseDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
