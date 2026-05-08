use dodopayments_rust::DodoPaymentsClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let entitlement_id = "ent_xxxxxxxxxx";
    let grant_id = "grant_xxxxxxxxxx";

    client
        .entitlements()
        .id(entitlement_id)
        .grant_id(grant_id)
        .revoke()
        .send()
        .await?;

    println!("Entitlement grant revoked successfully");

    Ok(())
}
