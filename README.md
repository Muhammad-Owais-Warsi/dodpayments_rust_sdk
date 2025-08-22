## Dodo Payments Rust SDK
Rust sdk to interact with dodopayments REST API.

The REST API documentation can be found on [docs.dodopayments.com](https://docs.dodopayments.com).

# Installation

```rust
cargo add dodopayments_rust
```

# Usage

```js
use dodopayments_rust::{DodoPaymentsClient, DodoPaymentsClientBuilder, ResponseData};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client: DodoPaymentsClient = DodoPaymentsClientBuilder::new()
        .bearer_token("")
        .enviroment("https://test.dodopayments.com")
        .build()
        .unwrap();

    let mut query_params = HashMap::new();
    query_params.insert("status", "succeeded");

    let body = None;
    let ext_path = None;

    match client
        .payments()
        .list(Some(query_params), body, ext_path)
        .await
    {
        Ok(resp) => match resp {
            ResponseData::Text(text) => {
                println!("Text response: {}", text);
            }
            ResponseData::Blob(bytes) => {
                std::fs::write("invoice.pdf", &bytes).expect("Failed to write file");
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
}

```

You can checkout examples folder to view the usage of all the endpoints.
