use crate::{
    error::{RestError, Result},
    types::{
        CreateDepositAddressesRequest, CreateDepositAddressesResponse,
        CreateWithdrawalAddressesRequest, CreateWithdrawalAddressesResponse, GetQuoteRequest,
        GetQuoteResponse, GetSupportedAssetsResponse, GetTransactionStatusRequest,
        GetTransactionStatusResponse, ToQueryParams,
    },
};
use reqwest::{Client, Method, Request};
use serde::{Serialize, de::DeserializeOwned};

const BASE_URL: &str = "https://bridge.polymarket.com";

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

    async fn post<Body: Serialize, Res: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        body: &Body,
    ) -> Result<Res> {
        let path = format!("{}/{}", self.base_url, path);
        println!("{:?}", path);
        let request = self
            .client
            .request(Method::POST, path)
            .json(body)
            .build()
            .map_err(RestError::RequestError)?;
        self.request(request).await
    }

    async fn get<Params: Serialize, Res: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        params: &Params,
    ) -> Result<Res> {
        let parms = params.query_params();
        let path = format!("{}/{}{}", self.base_url, path, parms);
        println!("{:?}", path);
        let request = self
            .client
            .request(Method::GET, path)
            .build()
            .map_err(RestError::RequestError)?;
        self.request(request).await
    }

    pub async fn get_supported_assets(&self) -> Result<GetSupportedAssetsResponse> {
        self.get("supported-assets", &()).await
    }

    pub async fn get_quote(&self, args: &GetQuoteRequest) -> Result<GetQuoteResponse> {
        self.post("quote", args).await
    }

    pub async fn create_deposit_addresses(
        &self,
        args: &CreateDepositAddressesRequest,
    ) -> Result<CreateDepositAddressesResponse> {
        self.post("deposit", args).await
    }

    pub async fn create_withdrawal_addresses(
        &self,
        args: &CreateWithdrawalAddressesRequest,
    ) -> Result<CreateWithdrawalAddressesResponse> {
        self.post("withdraw", args).await
    }

    pub async fn get_transaction_status(
        &self,
        args: &GetTransactionStatusRequest,
    ) -> Result<GetTransactionStatusResponse> {
        self.get(&format!("status/{}", args.address), &()).await
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_supported_assets() {
        let client = BridgeClient::default();

        let response = client.get_supported_assets().await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_quote() {
        let client = BridgeClient::default();

        let args = &GetQuoteRequest {
            from_amount_base_unit: 10000000.to_string(),
            from_chain_id: 1.to_string(),
            from_token_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            recipient_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            to_chain_id: 10.to_string(),
            to_token_address: "0x7F5c764cBc14f9669B88837ca1490cCa17c31607".to_string(),
        };

        let response = client.get_quote(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_create_deposit_addresses() {
        let client = BridgeClient::default();

        let args = &CreateDepositAddressesRequest {
            address: "0x56687bf447db6ffa42ffe2204a05edaa20f55839".to_string(),
        };
        let response = client.create_deposit_addresses(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_create_withdrawal_addresses() {
        let client = BridgeClient::default();

        let args = &CreateWithdrawalAddressesRequest {
            address: "0x56687bf447db6ffa42ffe2204a05edaa20f55839".to_string(),
            to_chain_id: 1.to_string(),
            to_token_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            recipient_addr: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
        };

        let response = client.create_withdrawal_addresses(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_transaction_status() {
        let client = BridgeClient::default();

        let args = &GetTransactionStatusRequest {
            address: "bc1qs82vw5pczv9uj44n4npscldkdjgfjqu7x9mlna".to_string(),
        };

        let response = client.get_transaction_status(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
