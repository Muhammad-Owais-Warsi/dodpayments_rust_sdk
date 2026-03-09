use crate::{
    client::DodoPaymentsClient,
    models::{CreateMeterRequest, ListMetersResponse, MeterResponse},
    request_builder::RequestBuilder,
};
use reqwest::Method;

pub struct MetersApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> MetersApi<'client> {
    pub fn new(client: &'client DodoPaymentsClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> RequestBuilder<'client, ListMetersResponse, (), ()> {
        RequestBuilder::new(self.client, Method::GET, "/meters")
    }

    pub fn create(&self) -> RequestBuilder<'client, MeterResponse, (), CreateMeterRequest> {
        RequestBuilder::new(self.client, Method::POST, "/meters")
    }

    pub fn id(&self, meter_id: impl Into<String>) -> MeterByIdApi<'client> {
        MeterByIdApi {
            client: self.client,
            meter_id: meter_id.into(),
        }
    }
}

pub struct MeterByIdApi<'client> {
    client: &'client DodoPaymentsClient,
    meter_id: String,
}

impl<'client> MeterByIdApi<'client> {
    pub fn retrieve(&self) -> RequestBuilder<'client, MeterResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::GET,
            format!("/meters/{}", self.meter_id),
        )
    }

    pub fn archive(&self) -> RequestBuilder<'client, (), (), ()> {
        RequestBuilder::new(
            self.client,
            Method::DELETE,
            format!("/meters/{}", self.meter_id),
        )
    }

    pub fn unarchive(&self) -> RequestBuilder<'client, MeterResponse, (), ()> {
        RequestBuilder::new(
            self.client,
            Method::POST,
            format!("/meters/{}/unarchive", self.meter_id),
        )
    }
}
