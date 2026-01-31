use crate::{
    error::{RestError, Result},
    types::ToQueryParams,
};
use reqwest::{Client, Method, Request};
use serde::{Serialize, de::DeserializeOwned};

const BASE_URL: &str = "https://data-api.polymarket.com";

pub struct BridgeClient {
    pub client: Client,
    pub base_url: String,
}

impl Default for BridgeClient {
    fn default() -> Self {
        BridgeClient::new(BASE_URL)
    }
}

impl BridgeClient {
    pub fn new(base_url: impl Into<String>) -> BridgeClient {
        BridgeClient {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    async fn request<Response: DeserializeOwned>(&self, request: Request) -> Result<Response> {
        let response = self
            .client
            .execute(request)
            .await
            .map_err(RestError::RequestError)?;

        let status_code = response.status();

        if !status_code.is_success() {
            let message = response.text().await.unwrap_or_default();

            return Err(RestError::HttpError {
                status: status_code,
                body: message,
            });
        }

        let text = response.text().await.map_err(RestError::RequestError)?;

        let response =
            serde_json::from_str::<Response>(&text).map_err(|e| RestError::ParseError {
                error: e,
                raw: text.clone(),
            })?;

        Ok(response)
    }

    async fn post(&self) {}

    async fn get<Req: Serialize, Res: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        req: &Req,
    ) -> Result<Res> {
        let parms = req.query_params();
        let request = self
            .client
            .request(Method::GET, path)
            .build()
            .map_err(RestError::RequestError)?;
        self.request(request).await
    }

    pub async fn get_supported_assets(&self) -> Result<()> {
        Ok(())
        //self.get("");
    }
}
