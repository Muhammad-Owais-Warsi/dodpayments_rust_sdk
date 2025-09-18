use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct MetersApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> MetersApi<'client> {
    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::POST,
                "/meters",
                query_params,
                body,
                ext_path,
            )
            .await
    }

    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,        
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::GET,
                "/meters",
                query_params,
                body,
                ext_path,
            )
            .await        
    }

    pub async fn retrieve(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/meters", query_params, body, ext_path)
            .await
    }

    pub async fn archive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,           
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(
                Method::DELETE,
                "/meters",
                query_params,
                body,
                ext_path,
            )
            .await   
    }

    pub async fn unarchive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,           
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/unarchive", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::POST,
                "/meters",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await   
    }

}
