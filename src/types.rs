use serde::{Deserialize, Serialize};

/// Trait for converting request types to URL query parameters.
///
/// This trait is automatically implemented for all types that implement [`Serialize`].
/// It uses [`serde_html_form`] to serialize the struct fields into a query string.
/// Arrays are serialized as repeated keys (`key=val1&key=val2`).
pub trait ToQueryParams: Serialize {
    /// Converts the request to a URL query string.
    ///
    /// Returns an empty string if no parameters are set, otherwise returns
    /// a string starting with `?` followed by URL-encoded key-value pairs.
    fn query_params(&self) -> String {
        let params = serde_html_form::to_string(self).unwrap_or_default();

        if params.is_empty() {
            String::new()
        } else {
            format!("?{params}")
        }
    }
}

impl<T: Serialize> ToQueryParams for T {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSupportedAssetsResponse {
    supported_assets: Vec<SupportedAsset>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SupportedAsset {
    //todo: string as u32
    pub chain_id: String,
    pub chain_name: String,
    pub token: Token,
    pub min_checkout_usd: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteRequest {
    pub from_amount_base_unit: String,
    pub from_chain_id: String,
    pub from_token_address: String,
    pub recipient_address: String,
    pub to_chain_id: String,
    pub to_token_address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetQuoteResponse {
    pub est_checkout_time_ms: u64,
    pub est_fee_breakdown: FeeBreakdown,
    pub est_input_usd: f64,
    pub est_output_usd: f64,
    pub est_to_token_base_unit: String,
    pub quote_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeBreakdown {
    pub app_fee_label: String,
    pub app_fee_percent: f64,
    pub app_fee_usd: f64,
    pub fill_cost_percent: f64,
    pub fill_cost_usd: f64,
    pub gas_usd: f64,
    pub max_slippage: f64,
    pub min_received: f64,
    pub swap_impact: f64,
    pub swap_impact_usd: f64,
    pub total_impact: f64,
    pub total_impact_usd: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateDepositAddressesRequest {
    pub address: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateDepositAddressesResponse {
    pub note: String,
    pub address: CrossChainAddress,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CrossChainAddress {
    pub evm: String,
    pub svm: String,
    pub btc: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateWithdrawalAddressesRequest {
    pub address: String,
    pub to_chain_id: String,
    pub to_token_address: String,
    pub recipient_addr: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateWithdrawalAddressesResponse {
    pub address: CrossChainAddress,
    pub note: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetTransactionStatusRequest {
    #[serde(skip_serializing)]
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetTransactionStatusResponse {
    pub transactions: Vec<DepositTransaction>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositTransaction {
    pub from_chain_id: String,
    pub from_token_address: String,
    pub from_amount_base_unit: String,
    pub to_chain_id: String,
    pub to_token_address: String,
    pub status: String,
    pub tx_hash: String,
    pub created_time_ms: u64,
}
