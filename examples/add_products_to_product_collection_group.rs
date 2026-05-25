use dodopayments_rust::{
    models::{AddProductsToGroupRequest, GroupProduct},
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
    let group_id = "group_xxxxxxxxxx";

    let resp = client
        .product_collections()
        .id(collection_id)
        .group_id(group_id)
        .add_products_to_group()
        .body(AddProductsToGroupRequest {
            products: vec![GroupProduct {
                product_id: "prod_xxxxxxxxxx".to_string(),
                status: Some(true),
            }],
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
