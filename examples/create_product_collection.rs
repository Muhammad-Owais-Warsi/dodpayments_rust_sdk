use dodopayments_rust::{
    models::{CreateProductCollectionRequest, GroupProduct, ProductCollectionGroup},
    to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let group = ProductCollectionGroup {
        group_name: Some("Featured".to_string()),
        products: vec![GroupProduct {
            product_id: "prod_xxxxxxxxxx".to_string(),
            status: Some(true),
        }],
        status: Some(true),
    };

    let resp = client
        .product_collections()
        .create()
        .body(CreateProductCollectionRequest {
            name: "Starter Collection".to_string(),
            description: Some("Curated starter products".to_string()),
            groups: vec![group],
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
