use dodopayments_rust::{
    models::PatchCreditEntitlementRequest, to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let entitlement_id = "ent_xxxxxxxxxx";

    let resp = client
        .credit_entitlements()
        .id(entitlement_id)
        .update()
        .body(PatchCreditEntitlementRequest {
            name: Some("Updated Credits".to_string()),
            description: Some("Updated credit allocation".to_string()),
            unit: Some("credits".to_string()),
            price_per_unit: Some("1.00".to_string()),
            currency: None,
            expires_after_days: None,
            max_rollover_count: None,
            overage_behavior: None,
            overage_enabled: None,
            overage_limit: None,
            rollover_enabled: None,
            rollover_percentage: None,
            rollover_timeframe_count: None,
            rollover_timeframe_interval: None,
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
