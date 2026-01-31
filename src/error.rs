#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[source] reqwest::Error),

    #[error("HTTP {status} error: {body}")]
    HttpError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to parse JSON: {error}\nRaw response: {raw}")]
    ParseError {
        #[source]
        error: serde_json::Error,
        raw: String,
    },
}

pub type Result<T> = std::result::Result<T, RestError>;
