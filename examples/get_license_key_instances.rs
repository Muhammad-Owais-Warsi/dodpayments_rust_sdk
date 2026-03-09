use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let license_key_id = "lic_xxxxxxxxxx";

    let resp = client
        .licenses()
        .license_keys()
        .id(license_key_id)
        .license_key_instances()
        .list()
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
