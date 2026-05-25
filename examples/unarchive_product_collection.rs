use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let collection_id = "col_xxxxxxxxxx";

    let resp = client
        .product_collections()
        .id(collection_id)
        .unarchive()
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
