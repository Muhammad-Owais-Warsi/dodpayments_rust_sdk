use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let refund_id = "ref_xxxxxxxxxx";

    let resp = client.refunds().id(refund_id).retrieve().send().await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
