use crate::{
    error::WebRouteError, route_segment::Segment, to_segments::ToSegments, utils::struct_to_map,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WebRoute {
    segments: Vec<Segment>,
}

impl WebRoute {
    pub fn new<R: ToSegments>(route: R) -> Self {
        Self {
            segments: route.to_segments(),
        }
    }

    pub fn join<R: ToSegments>(&self, route: R) -> Self {
        Self {
            segments: [self.segments.clone(), route.to_segments()].concat(),
        }
    }

    pub fn as_definition_route<V: serde::Serialize>(&self) -> String {
        let definition_segments = self
            .segments
            .iter()
            .map(Segment::to_definition)
            .collect::<Vec<_>>();

        format!("/{}", definition_segments.join("/"))
    }

    pub fn as_populated_route<V: serde::Serialize>(
        &self,
        values: &V,
    ) -> Result<String, WebRouteError> {
        let values = struct_to_map(values).ok_or(WebRouteError::InvalidValues)?;

        let populated_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_populated(&values))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format!("/{}", populated_segments.join("/")))
    }

    pub(crate) fn segments(&self) -> Vec<Segment> {
        self.segments.clone()
    }
}
