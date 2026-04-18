use std::path::PathBuf;

use bon::Builder;

use crate::trait_async::AsyncTelegramApi;
use crate::Error;

/// Asynchronous [`AsyncTelegramApi`] implementation with [`cyper`]
#[derive(Debug, Clone, Builder)]
#[must_use = "Bot needs to be used in order to be useful"]
pub struct Bot {
    #[builder(into)]
    pub api_url: String,

    #[builder(default = default_client())]
    pub client: cyper::Client,
}

fn default_client() -> cyper::Client {
    let client_builder = cyper::ClientBuilder::new();

    #[cfg(not(target_arch = "wasm32"))]
    let client_builder = client_builder
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(500));

    client_builder.build().unwrap()
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

    async fn decode_response<Output>(response: cyper::Response) -> Result<Output, Error>
    where
        Output: serde::de::DeserializeOwned,
    {
        let success = response.status().is_success();
        let message = response.text().await?;
        if success {
            Ok(crate::json::decode(&message)?)
        } else {
            Err(Error::Api(crate::json::decode(&message)?))
        }
    }
}

impl From<cyper::Error> for Error {
    fn from(error: cyper::Error) -> Self {
        // Prevent leakage of the bot token as its within the path
        Self::HttpCyper(error.without_url())
    }
}

impl AsyncTelegramApi for Bot {
    type Error = Error;

    async fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{method}", self.api_url);
        let mut prepared_request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");
        if let Some(params) = params {
            let json_string = crate::json::encode(&params)?;
            prepared_request = prepared_request.body(json_string);
        }
        let response = prepared_request.send().await?;
        Self::decode_response(response).await
    }

    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    async fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(&str, PathBuf)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug + std::marker::Send,
        Output: serde::de::DeserializeOwned,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use cyper::multipart;
            use serde_json::Value;

            let json_string = crate::json::encode(&params)?;
            let json_struct: serde_json::Map<String, Value> =
                serde_json::from_str(&json_string).unwrap();
            let file_keys: Vec<&str> = files.iter().map(|(key, _)| *key).collect();

            let mut form = multipart::Form::new();
            for (key, val) in json_struct {
                if !file_keys.contains(&key.as_str()) {
                    form = match val {
                        Value::String(val) => form.text(key, val),
                        other => form.text(key, other.to_string()),
                    };
                }
            }

            for (parameter_name, file_path) in files {
                let file = cyper::fs::File::open(&file_path)
                    .await
                    .map_err(Error::ReadFile)?;
                let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
                let part = multipart::Part::stream(file).file_name(file_name);
                form = form.part(parameter_name.to_owned(), part);
            }

            let url = format!("{}/{method}", self.api_url);

            let response = self.client.post(url).multipart(form).send().await?;
            Self::decode_response(response).await
        }

        #[cfg(target_arch = "wasm32")]
        Err(Error::WasmHasNoFileSupportYet)
    }
}
