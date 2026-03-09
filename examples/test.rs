use dodopayments_rust::models::{AddonCartResponseItem, CreateBrandRequest, PatchBrandRequest};
use dodopayments_rust::{to_pretty_json, DodoPaymentsClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DodoPaymentsClientBuilder::new()
        .bearer_token("WeYhADH39QiHaYWF.yazz7KDFrk6xNkxuuFoYsOh74jGaHBagTJosfx46Mc9W-1S3")
        .enviroment("test_mode")
        .build()?;

    // let brand = client
    //     .test_brand()
    //     .create()
    //     .body(CreateBrandRequest {
    //         name: Some("test".to_string()),
    //         description: Some("testing sdk".to_string()),
    //         support_email: None,
    //         url: None,
    //         statement_descriptor: None,
    //     })
    //     .send()
    //     .await?;

    // let c = client.misc().list_supported_countries().send().await?;
    //
    // let invoice = client
    //     .payments()
    //     .id("pay_0Na3kKPtuVBv4uRhckzFe")
    //     .retrieve_invoice()
    //     .send()
    //     .await?;

    // std::fs::write("invoice.pdf", &invoice.0)?;

    Ok(())
}
