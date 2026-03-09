use crate::{
    client::DodoPaymentsClient,
    models::{
        CreateWebhookRequest, GetWebhookHeadersResponse, GetWebhookSecretResponse,
        ListWebhooksResponse, PatchWebhookRequest, WebhookDetails, WebhookHeadersReq,
    },
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct WebHooksApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> WebHooksApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListWebhooksResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/webhooks")
    }

    pub fn create(&self) -> RequestBuilder<'client, WebhookDetails, (), CreateWebhookRequest> {
        RequestBuilder::new(self.client, Method::POST, "/webhooks")
    }

    pub fn id(&self, webhook_id: impl Into<String>) -> WebHookByIdApi<'client> {
        WebHookByIdApi {
            client: self.client,
            webhook_id: webhook_id.into(),
        }
    }
}

pub struct WebHookByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    webhook_id: String,
}

impl<'client> WebHookByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, WebhookDetails, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/webhooks/{}", self.webhook_id),
        )
    }

    pub fn update(&self) -> RequestBuilder<'client, WebhookDetails, (), PatchWebhookRequest> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/webhooks/{}", self.webhook_id),
        )
    }

    pub fn delete(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/webhooks/{}", self.webhook_id),
        )
    }

    pub fn retrieve_webhook_headers(
        &self,
    ) -> RequestBuilder<'client, GetWebhookHeadersResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/webhooks/{}/headers", self.webhook_id),
        )
    }

    pub fn update_webhook_headers(
        &self,
    ) -> RequestBuilder<'client, GetWebhookHeadersResponse, (), WebhookHeadersReq> {
        RequestBuilder::new(
            self.client,
            Method::PATCH,
            format!("/webhooks/{}/headers", self.webhook_id),
        )
    }

    pub fn signing_key(&self) -> RequestBuilder<'client, GetWebhookSecretResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/webhooks/{}/secret", self.webhook_id),
        )
    }
}
