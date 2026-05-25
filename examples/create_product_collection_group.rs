use dodopayments_rust::{
    models::{GroupProduct, ProductCollectionGroup},
    to_pretty_json, DodoPaymentsClientBuilder,
};

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
        .create_group()
        .body(ProductCollectionGroup {
            group_name: Some("Featured".to_string()),
            products: vec![GroupProduct {
                product_id: "prod_xxxxxxxxxx".to_string(),
                status: Some(true),
            }],
            status: Some(true),
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
