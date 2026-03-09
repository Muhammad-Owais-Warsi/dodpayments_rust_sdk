use reqwest::Method;

use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateCreditEntitlementRequest, CreateLedgerEntryRequest, CreateLedgerEntryResponse,
        CreditEntitlementResponse, CustomerCreditBalanceResponse, ListBalancesResponse,
        ListCreditEntitlementsResponse, ListGrantsResponse, ListLedgerResponse,
        PatchCreditEntitlementRequest,
    },
    request_builder::RequestBuilder,
};

pub struct CreditEntitlementsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> CreditEntitlementsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListCreditEntitlementsResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/credit-entitlements")
    }

    pub fn create(
        &self,
    ) -> RequestBuilder<'client, CreditEntitlementResponse, (), CreateCreditEntitlementRequest>
    {
        RequestBuilder::new(self.client, Method::POST, "/credit-entitlements")
    }

    pub fn id(&self, entitlement_id: impl Into<String>) -> CreditEntitlementByIdApi<'client> {
        CreditEntitlementByIdApi {
            client: self.client,
            entitlement_id: entitlement_id.into(),
        }
    }
}

pub struct CreditEntitlementByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    entitlement_id: String,
}

impl<'client> CreditEntitlementByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, CreditEntitlementResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/credit-entitlements/{}", self.entitlement_id),
        )
    }

    pub fn update(
        &self,
    ) -> RequestBuilder<'client, CreditEntitlementResponse, (), PatchCreditEntitlementRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/credit-entitlements/{}", self.entitlement_id),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/credit-entitlements/{}", self.entitlement_id),
        )
    }

    pub fn undelete(&self) -> RequestBuilder<'client, CreditEntitlementResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/credit-entitlements/{}/undelete", self.entitlement_id),
        )
    }

    pub fn list_balances(&self) -> RequestBuilder<'client, ListBalancesResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/credit-entitlements/{}/balances", self.entitlement_id),
        )
    }

    pub fn customer_id(
        &self,
        customer_id: impl Into<String>,
    ) -> CreditEntitlementCustomerBalanceApi<'client> {
        CreditEntitlementCustomerBalanceApi {
            client: self.client,
            entitlement_id: self.entitlement_id.clone(),
            customer_id: customer_id.into(),
        }
    }
}

pub struct CreditEntitlementCustomerBalanceApi<'client> {
    client: &'client DodoPaymentsClient,
    entitlement_id: String,
    customer_id: String,
}

impl<'client> CreditEntitlementCustomerBalanceApi<'client> {
    pub fn retrieve_customer_balance(
        &self,
    ) -> RequestBuilder<'client, CustomerCreditBalanceResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!(
                "/credit-entitlements/{}/balances/{}",
                self.entitlement_id, self.customer_id
            ),
        )
    }

    pub fn list_customer_grants(&self) -> RequestBuilder<'client, ListGrantsResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!(
                "/credit-entitlements/{}/balances/{}/grants",
                self.entitlement_id, self.customer_id
            ),
        )
    }

    pub fn list_customer_ledger(&self) -> RequestBuilder<'client, ListLedgerResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!(
                "/credit-entitlements/{}/balances/{}/ledger",
                self.entitlement_id, self.customer_id
            ),
        )
    }

    pub fn create_customer_ledger_entry(
        &self,
    ) -> RequestBuilder<'client, CreateLedgerEntryResponse, (), CreateLedgerEntryRequest> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!(
                "/credit-entitlements/{}/balances/{}/ledger-entries",
                self.entitlement_id, self.customer_id
            ),
        )
    }
}
