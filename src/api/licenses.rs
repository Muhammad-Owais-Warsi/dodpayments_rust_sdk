use crate::{
    client::DodoPaymentsClient,
    models::{
        ActivateLicenseKeyRequest, ActivateLicenseKeyResponse, DeactivateLicenseKeyRequest,
        LicenseKeyInstanceResponse, LicenseKeyResponse, ListLicenseKeyInstancesResponse,
        ListLicenseKeysResponse, PatchLicenseKeyInstanceRequest, PatchLicenseKeyRequest,
        ValidateLicenseKeyRequest, ValidateLicenseKeyResponse,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct LicensesApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> LicensesApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn activate(
        &self,
    ) -> RequestBuilder<'client, ActivateLicenseKeyResponse, (), ActivateLicenseKeyRequest> {
        RequestBuilder::new(self.client, Method::POST, "/licenses/activate")
    }

    pub fn deactivate(
        &self,
    ) -> RequestBuilder<'client, LicenseKeyResponse, (), DeactivateLicenseKeyRequest> {
        RequestBuilder::new(self.client, Method::POST, "/licenses/deactivate")
    }

    pub fn validate(
        &self,
    ) -> RequestBuilder<'client, ValidateLicenseKeyResponse, (), ValidateLicenseKeyRequest> {
        RequestBuilder::new(self.client, Method::POST, "/licenses/validate")
    }

    pub fn license_keys(&self) -> LicenseKeysApi<'client> {
        LicenseKeysApi {
            client: self.client,
        }
    }
}

pub struct LicenseKeysApi<'client> {
    client: &'client DodoPaymentsClient,
}

impl<'client> LicenseKeysApi<'client> {
    pub fn list(&self) -> RequestBuilder<'client, ListLicenseKeysResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/license_keys")
    }

    pub fn id(&self, license_key_id: impl Into<String>) -> LicenseKeyByIdApi<'client> {
        LicenseKeyByIdApi {
            client: self.client,
            license_key_id: license_key_id.into(),
        }
    }
}

pub struct LicenseKeyByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    license_key_id: String,
}

impl<'client> LicenseKeyByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, LicenseKeyResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/license_keys/{}", self.license_key_id),
        )
    }

    pub fn update(
        &self,
    ) -> RequestBuilder<'client, LicenseKeyResponse, (), PatchLicenseKeyRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/license_keys/{}", self.license_key_id),
        )
    }

    pub fn license_key_instances(&self) -> LicenseKeyInstancesApi<'client> {
        LicenseKeyInstancesApi {
            client: self.client,
        }
    }
}

pub struct LicenseKeyInstancesApi<'client> {
    client: &'client DodoPaymentsClient,
}

impl<'client> LicenseKeyInstancesApi<'client> {
    pub fn list(&self) -> RequestBuilder<'client, ListLicenseKeyInstancesResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/license_key_instances")
    }

    pub fn id(&self, instance_id: impl Into<String>) -> LicenseKeyInstanceByIdApi<'client> {
        LicenseKeyInstanceByIdApi {
            client: self.client,
            instance_id: instance_id.into(),
        }
    }
}

pub struct LicenseKeyInstanceByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    instance_id: String,
}

impl<'client> LicenseKeyInstanceByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, LicenseKeyInstanceResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/license_key_instances/{}", self.instance_id),
        )
    }

    pub fn update(
        &self,
    ) -> RequestBuilder<'client, LicenseKeyInstanceResponse, (), PatchLicenseKeyInstanceRequest>
    {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/license_key_instances/{}", self.instance_id),
        )
    }
}
