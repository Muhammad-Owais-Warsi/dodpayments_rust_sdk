use dodopayments_rust::{
    models::{CreateCheckoutSessionRequest, ProductItemReq},
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
        .checkout_session()
        .create()
        .body(CreateCheckoutSessionRequest {
            product_cart: vec![ProductItemReq {
                product_id: "prod_xxxxxxxxxx".to_string(),
                quantity: 1,
                addons: None,
                amount: None,
            }],
            return_url: Some("https://example.com/success".to_string()),
            ..Default::default()
        })
        .send()
        .await?;

    println!("{}", to_pretty_json(&resp)?);

    Ok(())
}
