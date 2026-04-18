use crate::Error;

/// Shortcut for [`serde_json::to_string`] with [`crate::Error`].
pub fn encode<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + std::fmt::Debug,
{
    serde_json::to_string(value).map_err(|error| Error::JsonEncode {
        source: error,
        input: format!("{value:?}"),
    })
}
