//! Функции для работы с АПИ-сервером
use api_tools::client::{api_query::*, api_request::ApiRequest};
use crate::{data::structs::*, error::{self, Error}};

/// Вспомогательная функция для выполнения запроса к апи-серверу
pub fn fetch_query(
    request: &mut ApiRequest,
    database: impl Into<String>,
    sql: impl Into<String>,
) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql)), false);
    Ok(request.fetch(&query, true)?)
}
