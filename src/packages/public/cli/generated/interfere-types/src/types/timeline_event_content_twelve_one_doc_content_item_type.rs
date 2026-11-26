pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentTwelveOneDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentTwelveOneDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentTwelveOneDocContentItemTypeType,
}

impl TimelineEventContentTwelveOneDocContentItemType {
    pub fn builder() -> TimelineEventContentTwelveOneDocContentItemTypeBuilder {
        <TimelineEventContentTwelveOneDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentTwelveOneDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentTwelveOneDocContentItemTypeType>,
}

impl TimelineEventContentTwelveOneDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentTwelveOneDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveOneDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOneDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentTwelveOneDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentTwelveOneDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveOneDocContentItemType, BuildError> {
        Ok(TimelineEventContentTwelveOneDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
