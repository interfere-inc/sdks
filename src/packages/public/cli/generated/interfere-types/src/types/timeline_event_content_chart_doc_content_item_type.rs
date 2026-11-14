pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartDocContentItemType {
    #[serde(default)]
    pub attrs: TimelineEventContentChartDocContentItemTypeAttrs,
    pub r#type: TimelineEventContentChartDocContentItemTypeType,
}

impl TimelineEventContentChartDocContentItemType {
    pub fn builder() -> TimelineEventContentChartDocContentItemTypeBuilder {
        <TimelineEventContentChartDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemTypeBuilder {
    attrs: Option<TimelineEventContentChartDocContentItemTypeAttrs>,
    r#type: Option<TimelineEventContentChartDocContentItemTypeType>,
}

impl TimelineEventContentChartDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: TimelineEventContentChartDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TimelineEventContentChartDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](TimelineEventContentChartDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemType, BuildError> {
        Ok(TimelineEventContentChartDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
