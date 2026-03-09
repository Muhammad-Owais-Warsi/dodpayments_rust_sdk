use dodopayments_rust::{
    models::{CreateLedgerEntryRequest, LedgerEntryType},
    to_pretty_json, DodoPaymentsClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("DODO_API_KEY")?;

    let client = DodoPaymentsClientBuilder::new()
        .bearer_token(&api_key)
        .enviroment("test_mode")
        .build()?;

    let entitlement_id = "ent_xxxxxxxxxx";
    let customer_id = "cus_xxxxxxxxxx";

    let resp = client
        .credit_entitlements()
        .id(entitlement_id)
        .customer_id(customer_id)
        .create_customer_ledger_entry()
        .body(CreateLedgerEntryRequest {
            amount: "100".to_string(),
            entry_type: LedgerEntryType::Credit,
            reason: None,
            idempotency_key: None,
            expires_at: None,
            metadata: None,
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
