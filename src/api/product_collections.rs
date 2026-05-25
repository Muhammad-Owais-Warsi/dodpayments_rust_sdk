use crate::{
    client::DodoPaymentsClient,
    models::{
        AddProductsToGroupRequest, CreateProductCollectionRequest,
        PatchProductCollectionGroupProductRequest, ProductCollectionGroup,
        ProductCollectionGroupResponse, ProductCollectionListItem,
        ProductCollectionProductResponse, ProductCollectionResponse, UndeleteCollectionResponse,
        UpdateCollectionImageResponse,
    },
    request_builder::{RawBytes, RequestBuilder},
};
use reqwest::Method;

pub struct ProductCollectionsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> ProductCollectionsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ProductCollectionListItem, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/product-collections")
    }

    pub fn create(
        &self,
    ) -> RequestBuilder<'client, ProductCollectionResponse, (), CreateProductCollectionRequest>
    {
        RequestBuilder::new(self.client, Method::POST, "/product-collections")
    }

    pub fn id(&self, id: impl Into<String>) -> ProductCollectionsByIdApi<'client> {
        ProductCollectionsByIdApi {
            client: self.client,
            id: id.into(),
        }
    }
}

pub struct ProductCollectionsByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    id: String,
}

impl<'client> ProductCollectionsByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, ProductCollectionResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/product-collections/{}", self.id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/product-collections/{}", self.id),
        )
    }

    pub fn archive(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/product-collections/{}", self.id),
        )
    }

    pub fn unarchive(&self) -> RequestBuilder<'client, UndeleteCollectionResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/product-collections/{}/unarchive", self.id),
        )
    }

    pub fn update_images(&self) -> RequestBuilder<'client, UpdateCollectionImageResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PUT,
            format!("/product-collections/{}/images", self.id),
        )
    }

    pub fn create_group(
        &self,
    ) -> RequestBuilder<'client, ProductCollectionGroupResponse, (), ProductCollectionGroup> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/product-collections/{}/groups", self.id),
        )
    }

    pub fn group_id(&self, group_id: impl Into<String>) -> ProductCollectionsGroupByIdApi<'client> {
        ProductCollectionsGroupByIdApi {
            client: self.client,
            id: self.id.clone(),
            group_id: group_id.into(),
        }
    }
}

pub struct ProductCollectionsGroupByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    id: String,
    group_id: String,
}

impl<'client> ProductCollectionsGroupByIdApi<'client> {
    pub fn update(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/product-collections/{}/groups/{}", self.id, self.group_id),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/product-collections/{}/groups/{}", self.id, self.group_id),
        )
    }

    pub fn add_products_to_group(
        &self,
    ) -> RequestBuilder<'client, Vec<ProductCollectionProductResponse>, (), AddProductsToGroupRequest>
    {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!(
                "/product-collections/{}/groups/{}/items",
                self.id, self.group_id
            ),
        )
    }

    pub fn item_id(
        &self,
        item_id: impl Into<String>,
    ) -> ProductCollectionsGroupItemByIdApi<'client> {
        ProductCollectionsGroupItemByIdApi {
            client: self.client,
            id: self.id.clone(),
            group_id: self.group_id.clone(),
            item_id: item_id.into(),
        }
    }
}

pub struct ProductCollectionsGroupItemByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    id: String,
    group_id: String,
    item_id: String,
}

impl<'client> ProductCollectionsGroupItemByIdApi<'client> {
    pub fn update(
        &self,
    ) -> RequestBuilder<'client, (), (), PatchProductCollectionGroupProductRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!(
                "/product-collections/{}/groups/{}/items/{}",
                self.id, self.group_id, self.item_id
            ),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, RawBytes, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!(
                "/product-collections/{}/groups/{}/items/{}",
                self.id, self.group_id, self.item_id
            ),
        )
    }
}
