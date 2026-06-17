use dodopayments_rust::{
    models::FulfillLicenseKeyRequest, to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let grant_id = "grant_xxxxxxxxxx";

    let resp = client
        .entitlements()
        .grant_id(grant_id)
        .fulfill_license_key_grant()
        .body(FulfillLicenseKeyRequest::new(
            "LICENSE-KEY-12345".to_string(),
        ))
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
