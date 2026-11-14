pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentElevenOneDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentElevenOneDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentElevenOneDocContentItemTypeType,
}

impl TimelineEventContentElevenOneDocContentItemType {
    pub fn builder() -> TimelineEventContentElevenOneDocContentItemTypeBuilder {
        <TimelineEventContentElevenOneDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentElevenOneDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentElevenOneDocContentItemTypeType>,
}

impl TimelineEventContentElevenOneDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentElevenOneDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentElevenOneDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOneDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentElevenOneDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentElevenOneDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentElevenOneDocContentItemType, BuildError> {
        Ok(TimelineEventContentElevenOneDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
