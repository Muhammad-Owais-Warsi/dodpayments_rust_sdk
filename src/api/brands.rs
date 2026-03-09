use crate::{
    client::DodoPaymentsClient,
    models::{
        BrandResponse, CreateBrandRequest, ListBrandsResponse, PatchBrandRequest,
        UpdateBrandImageResponse,
    },
    request_builder::RequestBuilder,
};

use reqwest::Method;

pub struct BrandsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> BrandsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListBrandsResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/brands")
    }

    pub fn create(&self) -> RequestBuilder<'client, BrandResponse, (), CreateBrandRequest> {
        RequestBuilder::new(self.client, Method::POST, "/brands")
    }

    pub fn id(&self, brand_id: impl Into<String>) -> BrandByIdApi<'client> {
        BrandByIdApi {
            client: self.client,
            brand_id: brand_id.into(),
        }
    }
}

pub struct BrandByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    brand_id: String,
}

impl<'client> BrandByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, BrandResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/brands/{}", self.brand_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, BrandResponse, (), PatchBrandRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/brands/{}", self.brand_id),
        )
    }

    pub fn update_brand_image(&self) -> RequestBuilder<'client, UpdateBrandImageResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PUT,
            format!("/brands/{}/images", self.brand_id),
        )
    }
}
