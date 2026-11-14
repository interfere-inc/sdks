pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartDocContentItemZeroContentItemThree {
    pub r#type: TimelineEventContentChartDocContentItemZeroContentItemThreeType,
}

impl TimelineEventContentChartDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentChartDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentChartDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<TimelineEventContentChartDocContentItemZeroContentItemThreeType>,
}

impl TimelineEventContentChartDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TimelineEventContentChartDocContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentChartDocContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentChartDocContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
