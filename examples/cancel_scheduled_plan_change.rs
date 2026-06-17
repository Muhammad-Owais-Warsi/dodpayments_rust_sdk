use dodopayments_rust::DodoPaymentsClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let subscription_id = "sub_xxxxxxxxxx";

    client
        .subscriptions()
        .id(subscription_id)
        .cancel_schedule_plan_change()
        .send()
        .await?;

    println!("Scheduled plan change cancelled successfully");

    Ok(())
}
