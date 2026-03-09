use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateDiscountRequest, DiscountResponse, GetDiscountsListResponse, PatchDiscountRequest,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct DiscountsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> DiscountsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetDiscountsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/discounts")
    }

    pub fn create(&self) -> RequestBuilder<'client, DiscountResponse, (), CreateDiscountRequest> {
        RequestBuilder::new(self.client, Method::POST, "/discounts")
    }

    pub fn id(&self, discount_id: impl Into<String>) -> DiscountByIdApi<'client> {
        DiscountByIdApi {
            client: self.client,
            discount_id: discount_id.into(),
        }
    }

    pub fn code(&self, discount_code: impl Into<String>) -> DiscountByCodeApi<'client> {
        DiscountByCodeApi {
            client: self.client,
            discount_code: discount_code.into(),
        }
    }
}

pub struct DiscountByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    discount_id: String,
}

impl<'client> DiscountByIdApi<'client> {
    pub fn validate(&self) -> RequestBuilder<'client, DiscountResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/discounts/{}", self.discount_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, DiscountResponse, (), PatchDiscountRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/discounts/{}", self.discount_id),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/discounts/{}", self.discount_id),
        )
    }
}

pub struct DiscountByCodeApi<'client> {
    client: &'client DodoPaymentsClient,
    discount_code: String,
}

impl<'client> DiscountByCodeApi<'client> {
    pub fn retrieve_by_code(&self) -> RequestBuilder<'client, DiscountResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/discounts/code/{}", self.discount_code),
        )
    }
}
