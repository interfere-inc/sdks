pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDiagram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc>,
    #[serde(rename = "compactText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_text: Option<String>,
    #[serde(rename = "seenAgain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_again: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChart {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder {
    chart: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart>,
    diagram: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDiagram>,
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc>,
    compact_text: Option<String>,
    seen_again: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain>,
    text: Option<String>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder {
    pub fn chart(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartChart) -> Self {
        self.chart = Some(value);
        self
    }

    pub fn diagram(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDiagram) -> Self {
        self.diagram = Some(value);
        self
    }

    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn compact_text(mut self, value: impl Into<String>) -> Self {
        self.compact_text = Some(value.into());
        self
    }

    pub fn seen_again(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain) -> Self {
        self.seen_again = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder::text)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChart, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChart {
            chart: self.chart,
            diagram: self.diagram,
            doc: self.doc,
            compact_text: self.compact_text,
            seen_again: self.seen_again,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
