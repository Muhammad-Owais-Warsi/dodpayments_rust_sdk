use crate::{
    client::DodoPaymentsClient,
    models::{CalculateSessionResponse, CreateCheckoutSessionRequest, CreateSessionResponse},
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct CheckoutApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> CheckoutApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn create(
        &self,
    ) -> RequestBuilder<'client, CreateSessionResponse, (), CreateCheckoutSessionRequest> {
        RequestBuilder::new(self.client, Method::POST, "/checkouts")
    }

    pub fn preview(
        &self,
    ) -> RequestBuilder<'client, CalculateSessionResponse, (), CreateCheckoutSessionRequest> {
        RequestBuilder::new(self.client, Method::POST, "/checkouts/preview")
    }

    pub fn id(&self, checkout_id: impl Into<String>) -> CheckoutByIdApi<'client> {
        CheckoutByIdApi {
            client: self.client,
            checkout_id: checkout_id.into(),
        }
    }
}

pub struct CheckoutByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    checkout_id: String,
}

impl<'client> CheckoutByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, CreateSessionResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/checkouts/{}", self.checkout_id),
        )
    }
}
