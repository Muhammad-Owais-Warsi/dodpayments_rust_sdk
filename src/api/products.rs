use crate::client::{DodoError, DodoPaymentsClient, ResponseData};
use reqwest::Method;
use std::collections::HashMap;

pub struct ProductsApi<'client> {
    pub(crate) client: &'client DodoPaymentsClient,
}

impl<'client> ProductsApi<'client> {
    pub async fn list(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/products", query_params, body, ext_path)
            .await
    }

    pub async fn create(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/products", query_params, body, ext_path)
            .await
    }

    pub async fn retreive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::GET, "/products", query_params, body, ext_path)
            .await
    }

    pub async fn update(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::PATCH, "/products", query_params, body, ext_path)
            .await
    }

    pub async fn update_product_image(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/images", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::PUT,
                "/products",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn delete(
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
                Method::DELETE,
                "/products",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }

    pub async fn unarchive(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        self.client
            .request(Method::POST, "/products", query_params, body, ext_path)
            .await
    }

    pub async fn update_files(
        &self,
        query_params: Option<HashMap<&str, &str>>,
        body: Option<serde_json::Value>,
        ext_path: Option<&str>,
    ) -> Result<ResponseData, DodoError> {
        let new_ext_path = match ext_path {
            Some(p) => format!("{}/files", p),
            None => {
                return Err(DodoError::Custom {
                    message: "Ext path not found".to_string(),
                });
            }
        };

        self.client
            .request(
                Method::PUT,
                "/products",
                query_params,
                body,
                Some(&new_ext_path),
            )
            .await
    }
}
