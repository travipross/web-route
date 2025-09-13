/// Represents an individual segment of a route (i.e. the bit between the `/`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fake", derive(fake::Dummy))]
pub struct WebSegment(String);

impl WebSegment {
    /// Returns the value of the segment.
    pub(crate) fn to_evaluated(&self) -> String {
        self.0.clone()
    }
}

/// It is often a path of insecure traversals if there are two consecutive slashes in a path. Making an empty [`WebSegment`] impossible to create removes the chance of consecutive slashes.
impl TryFrom<&str> for WebSegment {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for WebSegment {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}
