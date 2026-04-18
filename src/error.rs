use super::response::ErrorResponse;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Api Error {0:?}")]
    Api(ErrorResponse),

    #[cfg(feature = "client-cyper")]
    #[error("JSON Encode Error: {source} on {input}")]
    JsonEncode {
        source: serde_json::Error,
        input: String,
    },

    #[error("Read File Error: {0}")]
    ReadFile(#[source] std::io::Error),

    #[cfg(feature = "client-cyper")]
    #[error("HTTP error: {0}")]
    HttpCyper(#[source] cyper::Error),
}

impl Error {
    #[cfg(test)]
    #[track_caller]
    pub(crate) fn unwrap_api(self) -> ErrorResponse {
        if let Self::Api(api) = self {
            api
        } else {
            panic!("API Error expected: {self}");
        }
    }
}
