use dodopayments_rust::{models::PatchBrandRequest, to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let brand_id = "brand_xxxxxxxxxx";

    let resp = client
        .brands()
        .id(brand_id)
        .update()
        .body(PatchBrandRequest {
            name: Some("Updated Brand".to_string()),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
