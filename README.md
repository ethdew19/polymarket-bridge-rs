# polymarket-bridge-rs

A Rust wrapper for the Polymarket Bridge API, enabling cross-chain token transfers for deposits and withdrawals. Bridge assets from various chains (EVM, Solana, Bitcoin) to fund your Polymarket account.

## Features

- **Supported Assets**: Query available tokens and chains for bridging
- **Quotes**: Get estimated fees and output amounts before bridging
- **Deposits**: Generate deposit addresses for funding your Polymarket account
- **Withdrawals**: Create withdrawal addresses to move funds off Polymarket
- **Transaction Status**: Track the status of your bridge transactions

**Note**: This wrapper provides access to Polymarket's Bridge API for cross-chain transfers. For the gamma API, see [polymarket-gamma-rs](https://github.com/ethdew19/polymarket-gamma-rs). For the data API, see [polymarket-data-rs](https://github.com/ethdew19/polymarket-data-rs).

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
polymarket-bridge-rs = { git = "https://github.com/ethdew19/polymarket-bridge-rs" }
tokio = { version = "1", features = ["full"] }
```

Example Usage:

```rust
use polymarket_bridge_rs::client::BridgeClient;
use polymarket_bridge_rs::types::CreateDepositAddressesRequest;
use polymarket_bridge_rs::error::RestError;

#[tokio::main]
async fn main() -> Result<(), RestError> {
    // Create a new client
    let client = BridgeClient::default();

    // Get supported assets for bridging
    let assets = client.get_supported_assets().await?;
    println!("Supported assets: {:?}", assets);

    // Generate deposit addresses for your Polymarket account
    let args = CreateDepositAddressesRequest {
        address: "0xYourPolymarketAddress".to_string(),
    };
    let deposit_addrs = client.create_deposit_addresses(&args).await?;
    println!("Deposit addresses: {:?}", deposit_addrs);

    Ok(())
}
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
