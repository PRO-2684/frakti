use std::path::PathBuf;
use std::time::Duration;

use bon::Builder;
use multipart::client::lazy::Multipart;
use serde_json::Value;

use crate::trait_sync::TelegramApi;
use crate::Error;

/// Synchronous [`TelegramApi`] implementation with [`ureq`].
#[derive(Debug, Clone, Builder)]
#[must_use = "Bot needs to be used in order to be useful"]
pub struct Bot {
    #[builder(into)]
    pub api_url: String,

    #[builder(default = default_agent())]
    pub request_agent: ureq::Agent,
}

fn default_agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(500)))
            .build(),
    )
}

impl Bot {
    /// Create a new `Bot`. You can use [`Bot::new_url`] or [`Bot::builder`] for more options.
    pub fn new(api_key: &str) -> Self {
        Self::new_url(format!("{}{api_key}", crate::BASE_API_URL))
    }

    /// Create a new `Bot`. You can use [`Bot::builder`] for more options.
    pub fn new_url<S: Into<String>>(api_url: S) -> Self {
        Self::builder().api_url(api_url).build()
    }

    fn decode_response<Output>(
        response: ureq::http::response::Response<ureq::Body>,
    ) -> Result<Output, Error>
    where
        Output: serde::de::DeserializeOwned,
    {
        let success = response.status().is_success();
        let body = response.into_body().read_to_string()?;
        if success {
            crate::json::decode(&body)
        } else {
            let api_error = crate::json::decode(&body)?;
            Err(Error::Api(api_error))
        }
    }
}

impl TelegramApi for Bot {
    type Error = Error;

    fn request<Params, Output>(&self, method: &str, params: Option<Params>) -> Result<Output, Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{method}", self.api_url);
        let request = self.request_agent.post(&url);
        let response = match params {
            None => request.send_empty()?,
            Some(data) => {
                let json = crate::json::encode(&data)?;
                request
                    .header(
                        ureq::http::header::CONTENT_TYPE,
                        ureq::http::HeaderValue::from_static("application/json; charset=utf-8"),
                    )
                    .send(&json)?
            }
        };
        Self::decode_response(response)
    }

    fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        let json_string = crate::json::encode(&params)?;
        let json_struct: serde_json::Map<String, Value> =
            serde_json::from_str(&json_string).unwrap();
        let file_keys: Vec<&str> = files.iter().map(|(key, _)| *key).collect();

        let mut form = Multipart::new();
        for (key, val) in json_struct {
            if !file_keys.contains(&key.as_str()) {
                match val {
                    Value::String(val) => form.add_text(key, val),
                    other => form.add_text(key, other.to_string()),
                };
            }
        }

        for (parameter_name, file_path) in &files {
            let file = std::fs::File::open(file_path).map_err(Error::ReadFile)?;
            let file_name = file_path.file_name().unwrap().to_string_lossy();
            let file_extension = file_path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            let mime = mime_guess::from_ext(file_extension).first_or_octet_stream();
            form.add_stream(*parameter_name, file, Some(file_name), Some(mime));
        }

        let url = format!("{}/{method}", self.api_url);
        let mut form_data = form.prepare().unwrap();
        let response = self
            .request_agent
            .post(&url)
            .header(
                ureq::http::header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", form_data.boundary()),
            )
            .send(ureq::SendBody::from_reader(&mut form_data))?;
        Self::decode_response(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_correct_url() {
        let api = Bot::new("hey");
        assert_eq!("https://api.telegram.org/bothey", api.api_url);
    }
}
