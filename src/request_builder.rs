use crate::{client::DodoPaymentsClient, DodoError};
use reqwest::{Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub trait ApiResponse: Sized {
    async fn handle_response(resp: Response) -> Result<Self, DodoError>;
}

impl<T: DeserializeOwned> ApiResponse for T {
    async fn handle_response(resp: Response) -> Result<Self, DodoError> {
        Ok(resp.json::<T>().await?)
    }
}

pub struct RawBytes(pub bytes::Bytes);

impl ApiResponse for RawBytes {
    async fn handle_response(resp: Response) -> Result<Self, DodoError> {
        Ok(RawBytes(resp.bytes().await?))
    }
}

pub struct RequestBuilder<'a, TResp, TQuery = (), TBody = ()> {
    client: &'a DodoPaymentsClient,
    method: Method,
    path: String,
    query: Option<TQuery>,
    body: Option<TBody>,
    _resp: PhantomData<TResp>,
}

impl<'a, TResp, TQuery, TBody> RequestBuilder<'a, TResp, TQuery, TBody> {
    pub fn new(client: &'a DodoPaymentsClient, method: Method, path: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path: path.into(),
            query: None,
            body: None,
            _resp: PhantomData,
        }
    }
}

impl<'a, TResp, TQuery, TBody> RequestBuilder<'a, TResp, TQuery, TBody> {
    pub fn query<Q: Serialize>(self, query: Q) -> RequestBuilder<'a, TResp, Q, TBody> {
        RequestBuilder {
            client: self.client,
            method: self.method,
            path: self.path,
            query: Some(query),
            body: self.body,
            _resp: PhantomData,
        }
    }

    pub fn body(self, body: TBody) -> Self
    where
        TBody: Serialize,
    {
        Self {
            client: self.client,
            method: self.method,
            path: self.path,
            query: self.query,
            body: Some(body),
            _resp: PhantomData,
        }
    }

    pub async fn send(self) -> Result<TResp, DodoError>
    where
        TResp: ApiResponse,
        TQuery: Serialize,
        TBody: Serialize,
    {
        let url = format!("{}{}", self.client.environment, self.path);

        let mut req = self
            .client
            .client
            .request(self.method, &url)
            .bearer_auth(&self.client.bearer_token);

        if let Some(q) = self.query {
            req = req.query(&q);
        }

        if let Some(b) = self.body {
            req = req.json(&b);
        }

        let resp = req.send().await?;

        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            let message = if text.trim().is_empty() {
                status
                    .canonical_reason()
                    .unwrap_or("Request failed")
                    .to_string()
            } else {
                text
            };

            return Err(DodoError::Api {
                status: status.as_u16(),
                message,
            });
        }

        TResp::handle_response(resp).await
    }
}
