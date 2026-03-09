use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let payment_id = "pay_xxxxxxxxxx";

    let invoice = client
        .payments()
        .id(payment_id)
        .retrieve_invoice()
        .send()
        .await?;

    std::fs::write("invoice.pdf", &invoice.0)?;

    println!("Invoice saved to invoice.pdf");

    Ok(())
}
