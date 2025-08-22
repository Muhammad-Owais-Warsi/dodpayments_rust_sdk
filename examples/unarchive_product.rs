use dodopayments_rust::{DodoPaymentsClient, DodoPaymentsClientBuilder, ResponseData};
use serde_json::json;

#[tokio::main]
async fn main() {
    let client: DodoPaymentsClient = DodoPaymentsClientBuilder::new()
        .bearer_token("")
        .enviroment("https://test.dodopayments.com")
        .build()
        .unwrap();

    let query_params = None;
    let body = Some(json!({
        "file_name": ""
    }));

    let ext_path = "id";

    match client
        .products()
        .update_files(query_params, body, Some(ext_path))
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
