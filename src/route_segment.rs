use std::collections::HashMap;

use crate::error::WebRouteError;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Token(String),
    Static(String),
}

impl Segment {
    pub(crate) fn to_definition(&self) -> String {
        match self {
            Segment::Token(key) => format!("{{{key}}}"),
            Segment::Static(value) => value.to_owned(),
        }
    }

    pub(crate) fn to_populated(
        &self,
        value_map: &HashMap<String, String>,
    ) -> Result<String, WebRouteError> {
        let populated = match self {
            Segment::Token(key) => value_map
                .get(key)
                .ok_or(WebRouteError::UnpopulatedToken(key.to_owned()))?
                .to_owned(),
            Segment::Static(value) => value.to_owned(),
        };

        Ok(populated)
    }
}

impl From<&str> for Segment {
    fn from(segment: &str) -> Self {
        let segment = segment.trim();

        if segment.starts_with('{') && segment.ends_with('}') {
            let key = &segment[1..segment.len() - 1].trim();
            Segment::Token(key.to_string())
        } else {
            Segment::Static(segment.to_string())
        }
    }
}
