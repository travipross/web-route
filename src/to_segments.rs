//! Defines what can be used to create and join [`WebRoute`]s.

use std::cell::LazyCell;

use crate::{WebRoute, segment::Segment};

pub trait ToSegments {
    /// Defines how to convert something into a [`Vec`] of [`Segment`]s.
    fn to_segments(&self) -> Vec<Segment>;
}

impl ToSegments for &str {
    fn to_segments(&self) -> Vec<Segment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToSegments for String {
    fn to_segments(&self) -> Vec<Segment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToSegments for WebRoute {
    fn to_segments(&self) -> Vec<Segment> {
        self.segments()
    }
}

impl ToSegments for &WebRoute {
    fn to_segments(&self) -> Vec<Segment> {
        self.segments()
    }
}

impl ToSegments for LazyCell<WebRoute> {
    fn to_segments(&self) -> Vec<Segment> {
        self.segments()
    }
}
