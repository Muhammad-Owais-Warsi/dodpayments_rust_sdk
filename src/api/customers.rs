use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateCustomerPortalSessionResponse, CreateCustomerRequest, CreateLedgerEntryRequest,
        CreateLedgerEntryResponse, CustomerResponse, CustomerWalletResponse,
        GetCustomersListResponse, ListLedgerResponse, PatchCustomerRequest,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct CustomersApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> CustomersApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetCustomersListResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/customers")
    }

    pub fn create(&self) -> RequestBuilder<'client, CustomerResponse, (), CreateCustomerRequest> {
        RequestBuilder::new(self.client, Method::POST, "/customers")
    }

    pub fn id(&self, customer_id: impl Into<String>) -> CustomerByIdApi<'client> {
        CustomerByIdApi {
            client: self.client,
            customer_id: customer_id.into(),
        }
    }
}

pub struct CustomerByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    customer_id: String,
}

impl<'client> CustomerByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, CustomerResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/customers/{}", self.customer_id),
        )
    }

    pub fn list_customer_wallets(&self) -> RequestBuilder<'client, CustomerWalletResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/customers/{}/wallets", self.customer_id),
        )
    }

    pub fn retrieve_customer_wallet_ledger_entries(
        &self,
    ) -> RequestBuilder<'client, ListLedgerResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/customers/{}/wallets/ledger-entries", self.customer_id),
        )
    }

    pub fn create_customer_wallet_ledger_entry(
        &self,
    ) -> RequestBuilder<'client, CreateLedgerEntryResponse, (), CreateLedgerEntryRequest> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/customers/{}/wallets/ledger-entries", self.customer_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, CustomerResponse, (), PatchCustomerRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/customers/{}", self.customer_id),
        )
    }

    pub fn create_customer_portal(
        &self,
    ) -> RequestBuilder<'client, CreateCustomerPortalSessionResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/customers/{}/customer-portal/session", self.customer_id),
        )
    }
}
