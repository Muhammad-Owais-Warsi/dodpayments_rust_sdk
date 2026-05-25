use dodopayments_rust::{
    models::PatchProductCollectionGroupProductRequest, DodoPaymentsClientBuilder,
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
    let item_id = "item_xxxxxxxxxx";

    client
        .product_collections()
        .id(collection_id)
        .group_id(group_id)
        .item_id(item_id)
        .update()
        .body(PatchProductCollectionGroupProductRequest { status: false })
        .send()
        .await?;

    println!("Product collection group item updated successfully");

    Ok(())
}
