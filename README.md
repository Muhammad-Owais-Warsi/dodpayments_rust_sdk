## Dodo Payments Rust SDK

Unofficial rust sdk to interact with dodopayments REST API.

The REST API documentation can be found on [docs.dodopayments.com](https://docs.dodopayments.com).

# Installation

```js
cargo add dodopayments_rust
```

# Usage

```rust
use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let resp = client.payments().list().send().await?;
    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}

```

# Examples

You can checkout examples folder to view the usage of all the endpoints.
To run any example run the following command in your root

```js
  cargo run --example <file_name>
```
