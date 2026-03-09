use crate::{
    client::DodoPaymentsClient, models::CountryCodeAlpha2, request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct MiscApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> MiscApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list_supported_countries(
        &self,
    ) -> RequestBuilder<'client, Vec<CountryCodeAlpha2>, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/checkout/supported_countries")
    }
}
