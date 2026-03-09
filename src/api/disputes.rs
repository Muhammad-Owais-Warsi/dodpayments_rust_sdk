use crate::{
    client::DodoPaymentsClient,
    models::{GetDiscountsListResponse, GetDisputeResponse},
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct DisputesApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> DisputesApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetDiscountsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/disputes")
    }

    pub fn id(&self, dispute_id: impl Into<String>) -> DisputeByIdApi<'client> {
        DisputeByIdApi {
            client: self.client,
            dispute_id: dispute_id.into(),
        }
    }
}

pub struct DisputeByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    dispute_id: String,
}

impl<'client> DisputeByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, GetDisputeResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/disputes/{}", self.dispute_id),
        )
    }
}
