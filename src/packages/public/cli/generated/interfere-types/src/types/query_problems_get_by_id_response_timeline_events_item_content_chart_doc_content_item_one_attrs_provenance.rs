pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenance {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenance {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenanceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenanceBuilder {
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenanceBuilder {

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenance`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenance, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemOneAttrsProvenance {
        })
    }
}
