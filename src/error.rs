#[derive(Debug, Clone, thiserror::Error)]
pub enum WebRouteError {
    /// When generating a populated route, the `value`s input needs to
    /// serialize into key-value pairs.
    #[error("values need to be able to serialize into a `serde_json::Value::Object`")]
    InvalidValue,

    /// When generating a populated route, there was no value to populate the
    /// specified parameter key.
    #[error("no value to populate parameter: {0}")]
    UnpopulatedParam(String),
}
