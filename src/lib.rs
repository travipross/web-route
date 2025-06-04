pub mod error;
pub mod route_segment;
pub mod to_segments;
mod utils;
pub mod web_route;

pub use to_segments::ToSegments;
pub use web_route::WebRoute;

// #[cfg(test)]
// mod web_route_tests {
//     use super::*;

//     #[test]
//     fn from_axum_route_should_create_correct_segments() {
//         // Arrange
//         let route = "/some/{route}/value";

//         // Act
//         let web_route = WebRoute::from_axum_route(route);

//         // Assert
//         assert_eq!(
//             web_route.segments,
//             vec![
//                 RouteSegment::Static("some".to_owned()),
//                 RouteSegment::Token("route".to_owned()),
//                 RouteSegment::Static("value".to_owned())
//             ]
//         );
//     }
// }
