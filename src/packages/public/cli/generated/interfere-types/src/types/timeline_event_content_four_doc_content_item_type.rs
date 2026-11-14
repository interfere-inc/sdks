pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentFourDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentFourDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentFourDocContentItemTypeType,
}

impl TimelineEventContentFourDocContentItemType {
    pub fn builder() -> TimelineEventContentFourDocContentItemTypeBuilder {
        <TimelineEventContentFourDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentFourDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentFourDocContentItemTypeType>,
}

impl TimelineEventContentFourDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentFourDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentFourDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentFourDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemType, BuildError> {
        Ok(TimelineEventContentFourDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
