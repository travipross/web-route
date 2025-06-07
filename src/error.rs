#[derive(Debug, Clone, thiserror::Error)]
pub enum WebRouteError {
    /// When generating a populated path there was no value to populate the
    /// specified token key.
    #[error("values need to be able to deserialize into a `serde_json::Value::Object`")]
    InvalidValues,

    /// When generating a populated path there was no value to populate the
    /// specified token key.
    #[error("no value to populate parameter: {0}")]
    UnpopulatedParam(String),
}
