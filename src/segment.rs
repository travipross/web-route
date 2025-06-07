use std::collections::HashMap;

use crate::error::WebRouteError;

/// Represents an individual segment of a route (i.e. the bit between the `/`).
///
/// Handles converting it between the templated representation of the segment,
/// and the populated version.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    NamedParam(String),
    CatchallParam(String),
    Static(String),
}

impl Segment {
    /// Converts the [`Segment`] into its "templated" representation so that it
    /// can be used in route definitions.
    pub(crate) fn to_template(&self) -> String {
        match self {
            Segment::NamedParam(named_param) => format!("{{{named_param}}}"),
            Segment::CatchallParam(catchall_param) => format!("{{*{catchall_param}}}"),
            Segment::Static(value) => value.to_owned(),
        }
    }

    /// Attempts to populate a parameter based [`Segment`] with a value in the
    /// `param_value_map`. If the [`Segment`] is static, it internal value will
    /// be returned.
    ///
    /// # Errors
    ///
    /// A [`WebRouteError::UnpopulatedParam`] if no matching entry was found in
    /// the `param_value_map`.
    pub(crate) fn to_populated(
        &self,
        param_value_map: &HashMap<String, String>,
    ) -> Result<String, WebRouteError> {
        let populated = match self {
            Segment::NamedParam(param) => param_value_map
                .get(param)
                .ok_or(WebRouteError::UnpopulatedParam(param.to_owned()))?
                .to_owned(),
            Segment::CatchallParam(param) => param_value_map
                .get(param)
                .ok_or(WebRouteError::UnpopulatedParam(param.to_owned()))?
                .to_owned(),
            Segment::Static(value) => value.to_owned(),
        };

        Ok(populated)
    }
}

impl From<&str> for Segment {
    fn from(segment: &str) -> Self {
        let segment = segment.trim();

        if segment.starts_with("{*") && segment.ends_with('}') {
            let param = segment.trim_start_matches("{*").trim_end_matches('}');
            Segment::CatchallParam(param.to_string())
        } else if segment.starts_with('{') && segment.ends_with('}') {
            let param = segment.trim_start_matches('{').trim_end_matches('}');
            Segment::NamedParam(param.to_string())
        } else {
            Segment::Static(segment.to_string())
        }
    }
}

#[cfg(test)]
mod segment_tests {
    use super::*;

    mod to_template {
        use super::*;

        #[test]
        fn should_correctly_template_named_parameter() {
            // Arrange
            let segment = Segment::NamedParam("named_param".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "{named_param}");
        }

        #[test]
        fn should_correctly_template_catchall_parameter() {
            // Arrange
            let segment = Segment::CatchallParam("catchall_param".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "{*catchall_param}");
        }

        #[test]
        fn should_correctly_template_static_segment() {
            // Arrange
            let segment = Segment::Static("static".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "static");
        }
    }

    mod from {
        use super::*;

        #[test]
        fn should_parse_named_parameter() {
            // Act
            let segment = Segment::from("{named_param}");

            // Assert
            assert!(matches!(segment, Segment::NamedParam(param) if param == "named_param"))
        }

        #[test]
        fn should_parse_catchall_parameter() {
            // Act
            let segment = Segment::from("{*catchall_param}");

            // Assert
            assert!(matches!(segment, Segment::CatchallParam(param) if param == "catchall_param"))
        }

        #[test]
        fn should_parse_static_segment() {
            // Act
            let segment = Segment::from("static");

            // Assert
            assert!(matches!(segment, Segment::Static(value) if value == "static"))
        }
    }
}
