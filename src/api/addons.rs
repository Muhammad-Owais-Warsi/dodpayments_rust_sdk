use crate::{
    client::DodoPaymentsClient,
    models::{
        AddonResponse, AddonsListResponse, CreateAddonRequest, PatchAddonRequest,
        UpdateAddonImageResponse,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct AddOnsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> AddOnsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, AddonsListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/addons")
    }

    pub fn create(&self) -> RequestBuilder<'client, AddonResponse, (), CreateAddonRequest> {
        RequestBuilder::new(self.client, Method::POST, "/addons")
    }

    pub fn id(&self, addon_id: impl Into<String>) -> AddOnByIdApi<'client> {
        AddOnByIdApi {
            client: self.client,
            addon_id: addon_id.into(),
        }
    }
}

pub struct AddOnByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    addon_id: String,
}

impl<'client> AddOnByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, AddonResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/addons/{}", self.addon_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, AddonResponse, (), PatchAddonRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/addons/{}", self.addon_id),
        )
    }

    pub fn update_image(&self) -> RequestBuilder<'client, UpdateAddonImageResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::PUT,
            format!("/addons/{}/images", self.addon_id),
        )
    }
}
