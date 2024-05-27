//! Структура с данными для ship_name и ship_parameters
use crate::error::Error;
use api_tools::client::api_query::*;
use api_tools::client::api_request::*;
//use serde_json::Value;

/// Структура с данными для ship_name и ship_parameters
pub struct General {
    data: String,
}
///
impl General {
    ///
    pub fn new(data: String) -> Self {
        Self { data }
    }
    /// Создание записи в таблице ship_name.
    /// Возвращает id судна
    pub fn process(self) -> Result<usize, Error> {
        let data: Vec<&str> = self.data.split("\r\n").filter(|s| s.len() > 0).collect();
        let mut parsed = Vec::new();
        for line in data {
            let vector = line.split(';').collect::<Vec<&str>>();
            if vector.len() != 3 {
                return Err(Error::FromString(format!(
                    "parse_general key/value error: {line}"
                )));
            }
            if vector[0].len() == 0 {
                return Err(Error::FromString(format!(
                    "parse_general key error: {line}"
                )));
            }
            if vector[1].len() == 0 {
                return Err(Error::FromString(format!(
                    "parse_general value error: {line}"
                )));
            }
            if vector[3].len() == 0 {
                return Err(Error::FromString(format!(
                    "parse_general value error: {line}"
                )));
            }
            parsed.push(vector);
        }
        let name = parsed
            .iter()
            .find(|v| v[0] == "Ship name")
            .ok_or(Error::FromString(format!("parse_general no Ship name!")))?[0];
        let project = parsed
            .iter()
            .find(|v| v[0] == "Project name")
            .unwrap_or(&vec!["NULL"])[0];
        let year_of_built = parsed
            .iter()
            .find(|v| v[0] == "Year of built")
            .unwrap_or(&vec!["NULL"])[0];
        let place_of_built = parsed
            .iter()
            .find(|v| v[0] == "Place of built")
            .unwrap_or(&vec!["NULL"])[0];
        let imo = parsed
            .iter()
            .find(|v| v[0] == "IMO number")
            .unwrap_or(&vec!["NULL"])[0];
        let mmsi = parsed
            .iter()
            .find(|v| v[0] == "MMSI")
            .unwrap_or(&vec!["NULL"])[0];
        let sql = format!(
            "INSERT INTO ship_name (name, project, year_of_built, place_of_built, IMO, MMSI) VALUES ({}, {}, {}, {}, {}, {});",
            name, project, year_of_built, place_of_built, imo, mmsi).to_owned();
        let mut request = ApiRequest::new(
            "parent",
            "0.0.0.0:8080",
            "auth_token",
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new("sss-computing", "")),
                false,
            ),
            false,
            false,
        );
        request.fetch(
            &ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new("sss-computing", sql)),
                false,
            ),
            true,
        )?;
        let result: serde_json::Value = serde_json::from_slice(
            &(request.fetch(
                &ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(
                        "sss-computing",
                        format!("SELECT id FROM ship_name WHERE name={name};").to_owned(),
                    )),
                    false,
                ),
                true,
            )?),
        )?;
        let id = result
            .get("id")
            .ok_or(Error::FromString(format!(
                "parse_general no ship_id in reply:{result}"
            )))?
            .as_u64()
            .ok_or(Error::FromString(format!(
                "parse_general wrong ship_id type in reply:{result}"
            )))?;
        let mut sql =
            "INSERT INTO ship_parameters (ship_id, key, value, value_type, unit) VALUES".to_owned();
        for line in parsed {
            let value_type = if line[2].parse::<f64>().is_ok() {
                "real"
            } else {
                "text"
            };
            sql += &format!(
                "({}, {}, {}, {}, {}),",
                id, line[0], line[2], value_type, line[1]
            )
            .to_owned();
        }
        request.fetch(
            &ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new("sss-computing", sql)),
                false,
            ),
            true,
        )?;
        Ok(id as usize)
    }
}
