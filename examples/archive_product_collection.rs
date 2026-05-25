use dodopayments_rust::DodoPaymentsClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let collection_id = "col_xxxxxxxxxx";

    client
        .product_collections()
        .id(collection_id)
        .archive()
        .send()
        .await?;

    println!("Product collection archived successfully");

    Ok(())
}
