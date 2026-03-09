use crate::{
    client::DodoPaymentsClient,
    models::{Event, GetEventsResponse, IngestEventsRequest, IngestEventsResponse},
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct UsageEventsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> UsageEventsApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, GetEventsResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/events")
    }

    pub fn ingest(&self) -> RequestBuilder<'client, IngestEventsResponse, (), IngestEventsRequest> {
        RequestBuilder::new(self.client, Method::POST, "/events/ingest")
    }

    pub fn id(&self, event_id: impl Into<String>) -> UsageEventByIdApi<'client> {
        UsageEventByIdApi {
            client: self.client,
            event_id: event_id.into(),
        }
    }
}

pub struct UsageEventByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    event_id: String,
}

impl<'client> UsageEventByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, Event, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/events/{}", self.event_id),
        )
    }
}
