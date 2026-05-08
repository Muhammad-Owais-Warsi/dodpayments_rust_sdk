use dodopayments_rust::{
    models::PatchEntitlementRequest, to_pretty_json, DodoPaymentsClientBuilder,
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
        .entitlements()
        .id(entitlement_id)
        .update()
        .body(PatchEntitlementRequest {
            name: Some(Some("Updated Entitlement".to_string())),
            description: Some(Some("Updated entitlement description".to_string())),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
