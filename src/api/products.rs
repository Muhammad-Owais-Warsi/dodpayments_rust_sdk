use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateProductRequest, CreateShortLinkRequest, GetProductResponse, GetProductsListResponse,
        ListShortLinksResponse, PatchProductRequest, ShortLinkResponse, UpdateProductImageResponse,
        UploadProductFile, UploadProductFileResponse,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct ProductsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> ProductsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetProductsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/products")
    }

    pub fn create(&self) -> RequestBuilder<'client, GetProductResponse, (), CreateProductRequest> {
        RequestBuilder::new(self.client, Method::POST, "/products")
    }

    pub fn list_short_links(&self) -> RequestBuilder<'client, ListShortLinksResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/products/short_links")
    }

    pub fn id(&self, product_id: impl Into<String>) -> ProductByIdApi<'client> {
        ProductByIdApi {
            client: self.client,
            product_id: product_id.into(),
        }
    }
}

pub struct ProductByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    product_id: String,
}

impl<'client> ProductByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, GetProductResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/products/{}", self.product_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, GetProductResponse, (), PatchProductRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/products/{}", self.product_id),
        )
    }

    pub fn update_image(&self) -> RequestBuilder<'client, UpdateProductImageResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PUT,
            format!("/products/{}/images", self.product_id),
        )
    }

    pub fn archive(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/products/{}", self.product_id),
        )
    }

    pub fn unarchive(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/products/{}/unarchive", self.product_id),
        )
    }

    pub fn update_files(
        &self,
    ) -> RequestBuilder<'client, UploadProductFileResponse, (), UploadProductFile> {
        RequestBuilder::new(
            self.client,
            Method::PUT,
            format!("/products/{}/files", self.product_id),
        )
    }

    pub fn create_short_link(
        &self,
    ) -> RequestBuilder<'client, ShortLinkResponse, (), CreateShortLinkRequest> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/products/{}/short_links", self.product_id),
        )
    }
}
