use dodopayments_rust::{
    models::DeactivateLicenseKeyRequest, to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let resp = client
        .licenses()
        .deactivate()
        .body(DeactivateLicenseKeyRequest {
            license_key: "lic_xxxxxxxxxx".to_string(),
            license_key_instance_id: "lki_xxxxxxxxxx".to_string(),
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
