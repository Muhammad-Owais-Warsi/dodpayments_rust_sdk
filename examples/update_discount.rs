use dodopayments_rust::{models::PatchDiscountRequest, to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let discount_id = "dis_xxxxxxxxxx";

    let resp = client
        .discounts()
        .id(discount_id)
        .update()
        .body(PatchDiscountRequest {
            amount: Some(15),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
