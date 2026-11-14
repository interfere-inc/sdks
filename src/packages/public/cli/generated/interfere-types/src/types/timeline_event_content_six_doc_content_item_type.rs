pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentSixDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentSixDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentSixDocContentItemTypeType,
}

impl TimelineEventContentSixDocContentItemType {
    pub fn builder() -> TimelineEventContentSixDocContentItemTypeBuilder {
        <TimelineEventContentSixDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentSixDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentSixDocContentItemTypeType>,
}

impl TimelineEventContentSixDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentSixDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentSixDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentSixDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemType, BuildError> {
        Ok(TimelineEventContentSixDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
