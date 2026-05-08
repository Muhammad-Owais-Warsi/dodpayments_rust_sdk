use crate::{
    client::DodoPaymentsClient,
    models::{
        list_entitlement_grants_response::ListEntitlementGrantsResponse, CreateEntitlementRequest,
        EntitlementResponse, ListEntitlementsResponse, PatchEntitlementRequest,
        UploadEntitlementFileResponse,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct EntitlementsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> EntitlementsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListEntitlementsResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/entitlements")
    }

    pub fn create(
        &self,
    ) -> RequestBuilder<'client, EntitlementResponse, (), CreateEntitlementRequest> {
        RequestBuilder::new(self.client, Method::POST, "/entitlements")
    }

    pub fn id(&self, entitlement_id: impl Into<String>) -> EntitlementsByIdApi<'client> {
        EntitlementsByIdApi {
            client: self.client,
            entitlement_id: entitlement_id.into(),
        }
    }
}

pub struct EntitlementsByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    entitlement_id: String,
}

impl<'client> EntitlementsByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, EntitlementResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/entitlements/{}", self.entitlement_id),
        )
    }

    pub fn update(
        &self,
    ) -> RequestBuilder<'client, EntitlementResponse, (), PatchEntitlementRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/entitlements/{}", self.entitlement_id),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/entitlements/{}", self.entitlement_id),
        )
    }

    pub fn upload(&self) -> RequestBuilder<'client, UploadEntitlementFileResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/entitlements/{}/files", self.entitlement_id),
        )
    }

    pub fn list_grants(&self) -> RequestBuilder<'client, ListEntitlementGrantsResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/entitlements/{}/grants", self.entitlement_id),
        )
    }

    pub fn file_id(&self, file_id: impl Into<String>) -> EntitlementsFileApi<'client> {
        EntitlementsFileApi {
            client: self.client,
            entitlement_id: self.entitlement_id.clone(),
            file_id: file_id.into(),
        }
    }

    pub fn grant_id(&self, grant_id: impl Into<String>) -> EntitlementsGrantApi<'client> {
        EntitlementsGrantApi {
            client: self.client,
            entitlement_id: self.entitlement_id.clone(),
            grant_id: grant_id.into(),
        }
    }
}

pub struct EntitlementsFileApi<'client> {
    client: &'client DodoPaymentsClient,
    entitlement_id: String,
    file_id: String,
}

impl<'client> EntitlementsFileApi<'client> {
    pub fn delete(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!(
                "/entitlements/{}/files/{}",
                self.entitlement_id, self.file_id
            ),
        )
    }
}

pub struct EntitlementsGrantApi<'client> {
    client: &'client DodoPaymentsClient,
    entitlement_id: String,
    grant_id: String,
}

impl<'client> EntitlementsGrantApi<'client> {
    pub fn revoke(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!(
                "/entitlements/{}/grants/{}",
                self.entitlement_id, self.grant_id
            ),
        )
    }
}
