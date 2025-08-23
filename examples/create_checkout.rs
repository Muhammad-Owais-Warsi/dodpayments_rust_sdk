use dodopayments_rust::{DodoPaymentsClient, DodoPaymentsClientBuilder, ResponseData};
use serde_json::json;

#[tokio::main]
async fn main() {
    let client: DodoPaymentsClient = DodoPaymentsClientBuilder::new()
        .bearer_token("")
        .enviroment("test_mode")
        .build()
        .unwrap();

    let query_params = None;
    let ext_path = None;

    let body = Some(json!({
        "product_cart": {

        },
        "customer": {

        },
        "return_url": ""
    }));

    match client
        .checkout_session()
        .create(query_params, body, ext_path)
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
