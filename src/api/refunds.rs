use crate::{
    client::DodoPaymentsClient,
    models::{CreateRefundRequest, GetRefundsListResponse, RefundResponse},
    request_builder::{RawBytes, RequestBuilder},
};
use reqwest::Method;

pub struct RefundsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> RefundsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetRefundsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/refunds")
    }

    pub fn create(&self) -> RequestBuilder<'client, RefundResponse, (), CreateRefundRequest> {
        RequestBuilder::new(self.client, Method::POST, "/refunds")
    }

    pub fn id(&self, refund_id: impl Into<String>) -> RefundByIdApi<'client> {
        RefundByIdApi {
            client: self.client,
            refund_id: refund_id.into(),
        }
    }
}

pub struct RefundByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    refund_id: String,
}

impl<'client> RefundByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, RefundResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/refunds/{}", self.refund_id),
        )
    }

    pub fn retrieve_reciept(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("invoices/refunds/{}", self.refund_id),
        )
    }
}
