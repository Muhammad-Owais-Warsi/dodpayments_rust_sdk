use dodopayments_rust::{
    models::CreateCreditEntitlementRequest, to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let resp = client
        .credit_entitlements()
        .create()
        .body(CreateCreditEntitlementRequest {
            name: "Basic Credits".to_string(),
            description: Some("Monthly credit allocation".to_string()),
            unit: "credits".to_string(),
            price_per_unit: Some("1.00".to_string()),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
