use super::Error;

/// Shortcut for [`serde_json::to_value`] producing a JSON object with [`crate::Error`].
pub fn encode_object<T>(value: &T) -> Result<serde_json::Map<String, serde_json::Value>, Error>
where
    T: serde::ser::Serialize + std::fmt::Debug,
{
    let json = serde_json::to_value(value).map_err(|error| Error::JsonEncode {
        source: error,
        input: format!("{value:?}"),
    })?;

    let serde_json::Value::Object(object) = json else {
        unreachable!("Telegram API params must serialize to a JSON object");
    };

    Ok(object)
}
