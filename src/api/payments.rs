use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateOneTimePaymentRequest, CreateOneTimePaymentResponse, GetPaymentsListResponse,
        PaymentLineItemsResponse, PaymentResponse,
    },
    request_builder::{RawBytes, RequestBuilder},
};
use reqwest::Method;

pub struct PaymentsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> PaymentsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetPaymentsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/payments")
    }

    #[deprecated(note = "This API will be deprecated soon. We recommend using Checkout Sessions.")]
    pub fn create(
        &self,
    ) -> RequestBuilder<'client, CreateOneTimePaymentResponse, (), CreateOneTimePaymentRequest>
    {
        RequestBuilder::new(self.client, Method::POST, "/payments")
    }

    pub fn id(&self, payment_id: impl Into<String>) -> PaymentByIdApi<'client> {
        PaymentByIdApi {
            client: self.client,
            payment_id: payment_id.into(),
        }
    }
}

pub struct PaymentByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    payment_id: String,
}

impl<'client> PaymentByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, PaymentResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/payments/{}", self.payment_id),
        )
    }

    pub fn retrieve_invoice(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/invoices/payments/{}", self.payment_id),
        )
    }

    pub fn line_items(&self) -> RequestBuilder<'client, PaymentLineItemsResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/payments/{}/line-items", self.payment_id),
        )
    }
}
