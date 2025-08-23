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
    let body = Some(json!({
        "currency": "",
    }));

    let ext_path = "id";

    match client
        .addons()
        .update(query_params, body, Some(ext_path))
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
