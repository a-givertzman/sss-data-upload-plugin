//! Функции для работы с АПИ-сервером
use crate::error::Error;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;
use serde_json::Value;

pub struct ApiServer {
    database: String,
    request: Option<ApiRequest>,
}
///
impl ApiServer {
    pub fn new(database: String) -> Self {
        Self {
            database,
            request: None,
        }
    }
    ///
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<Value>, Error> {
        if let Some(request) = self.request.as_mut() {
            let result = request.fetch(
                &ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)), false),
                true,
            )?;
            dbg!(sql, &String::from_utf8(result.clone()));
            let json: serde_json::Value = serde_json::from_slice(&result)?;
            let error_mess = json
                .get("error")
                .ok_or(format!("ApiServer can't get error:{}", json))?
                .get("message")
                .ok_or(format!("ApiServer can't get error message:{}", json))?
                .as_str()
                .ok_or(format!("ApiServer can't get error message str:{}", json))?;
            if error_mess.len() > 0 {
                return Err( Error::FromString( error_mess.to_owned() ));
            }
            let data = json
                .get("data")
                .ok_or(format!("ApiServer can't get data:{}", json))?
                .as_array()
                .ok_or(format!("ApiServer can't get data as_array:{}", json))?;
            Ok(data.to_owned())
        } else {
            self.request = Some(ApiRequest::new(
                "parent",
                "0.0.0.0:8080",
                "auth_token",
                ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), "")), false),
                true,
                false,
            ));
            self.fetch(sql)
        }
    }
}
