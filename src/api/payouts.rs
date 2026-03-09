use crate::{
    client::DodoPaymentsClient, models::GetPayoutsResponseList, request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct PayoutApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> PayoutApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetPayoutsResponseList, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/payouts")
    }
}
