//! Функции для работы с АПИ-сервером
use crate::error::Error;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;
use serde_json::Value;
use std::{thread, time};

pub struct ApiServer {
    database: String,
    request: Option<ApiRequest>,
}
//
impl ApiServer {
    pub fn new(database: String) -> Self {
        Self {
            database,
            request: None,
        }
    }
    //
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
        let mut request = ApiRequest::new(
            &api_tools::debug::dbg_id::DbgId("parent".to_owned()),
            "0.0.0.0:8080",
            "auth_token",
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)),
                false,
            ),
            true,
            false,
        );
        request.fetch(true).map_err(|e| Error::FromString(format!("ApiServer fetch error: {e}")))
    }
}
