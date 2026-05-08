use dodopayments_rust::{
    models::{
        CreateEntitlementRequest, EntitlementIntegrationType, IntegrationConfig, LicenseKeyConfig,
    },
    to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let resp = client
        .entitlements()
        .create()
        .body(CreateEntitlementRequest {
            name: "License Key Entitlement".to_string(),
            description: Some(Some("Access via license keys".to_string())),
            integration_type: EntitlementIntegrationType::LicenseKey,
            integration_config: Box::new(IntegrationConfig::LicenseKeyConfig(Box::new(
                LicenseKeyConfig::default(),
            ))),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
