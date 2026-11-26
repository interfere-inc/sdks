pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentTwelveCauseDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentTwelveCauseDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentTwelveCauseDocContentItemTypeType,
}

impl TimelineEventContentTwelveCauseDocContentItemType {
    pub fn builder() -> TimelineEventContentTwelveCauseDocContentItemTypeBuilder {
        <TimelineEventContentTwelveCauseDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentTwelveCauseDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentTwelveCauseDocContentItemTypeType>,
}

impl TimelineEventContentTwelveCauseDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentTwelveCauseDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentTwelveCauseDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCauseDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentTwelveCauseDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentTwelveCauseDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentTwelveCauseDocContentItemType, BuildError> {
        Ok(TimelineEventContentTwelveCauseDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
