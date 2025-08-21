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
    query_params.insert("page_size", "1");

    let body = None;
    let ext_path = None;

    match client
        .disputes()
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
