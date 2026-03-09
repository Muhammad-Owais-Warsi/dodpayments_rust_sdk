use reqwest::Method;

use crate::{
    models::ListBalanceLedgerEntriesResponse, request_builder::RequestBuilder, DodoPaymentsClient,
};

pub struct BalanceLedgerEntriesApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> BalanceLedgerEntriesApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListBalanceLedgerEntriesResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/balances/ledger")
    }
}
