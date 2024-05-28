//! Структура с данными для ship_name и ship_parameters
use crate::error::Error;
//use api_tools::client::api_query::*;
//use api_tools::client::api_request::*;
//use serde_json::Value;
use crate::ApiServer;

/// Структура с данными для ship_name и ship_parameters
pub struct General {
    data: String,
    api_server: ApiServer,
}
///
impl General {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            api_server: ApiServer::new("sss-computing".to_owned()),
        }
    }
    /// Создание записи в таблице ship_name.
    /// Возвращает id судна
    pub fn process(mut self) -> Result<usize, Error> {
        println!("General process begin");
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
            if vector[2].len() == 0 {
                return Err(Error::FromString(format!(
                    "parse_general value error: {line}"
                )));
            }
            parsed.push(vector);
        }
        println!("General parse ok");
        let name = format!(
            "'{}'",
            parsed
                .iter()
                .find(|v| v[0] == "Name of ship")
                .ok_or(Error::FromString(format!(
                    "parse_general no 'Name of ship'!"
                )))?[2]
        );
        let project = if let Some(v) = parsed.iter().find(|v| v[0] == "Name of project") {
            v[0]
        } else {
            "NULL"
        };
        let year_of_built = if let Some(v) = parsed.iter().find(|v| v[0] == "Year of built") {
            v[0]
        } else {
            "NULL"
        };
        let place_of_built = if let Some(v) = parsed.iter().find(|v| v[0] == "Place of built") {
            v[0]
        } else {
            "NULL"
        };
        let imo = if let Some(v) = parsed.iter().find(|v| v[0] == "IMO number") {
            v[0]
        } else {
            "NULL"
        };
        let mmsi = if let Some(v) = parsed.iter().find(|v| v[0] == "MMSI") {
            v[0]
        } else {
            "NULL"
        };
        self.api_server.fetch(&format!(
            "INSERT INTO ship_name AS s(name, project, year_of_built, place_of_built, IMO, MMSI) \
            VALUES ({name}, {project}, {year_of_built}, {place_of_built}, {imo}, {mmsi}) \
            ON CONFLICT ON CONSTRAINT ship_name_unique DO UPDATE \
            SET project={project}, year_of_built={year_of_built}, place_of_built={place_of_built}, IMO={imo}, MMSI={mmsi} \
            WHERE s.name={name};"
        ))?;
        let result = self
            .api_server
            .fetch(&format!("SELECT id FROM ship_name WHERE name={name};").to_owned())?;
        println!("General ship_name ok");
        let id = result[0]
            .get("id")
            .ok_or(Error::FromString(format!(
                "parse_general no ship_id in reply"
            )))?
            .as_u64()
            .ok_or(Error::FromString(format!(
                "parse_general wrong ship_id type in reply"
            )))?;
        println!("General ship_id ok");
        self
            .api_server
            .fetch(&format!("DELETE FROM ship_parameters WHERE ship_id={id};").to_owned())?;
        let mut sql =
            "INSERT INTO ship_parameters (ship_id, key, value, value_type, unit) VALUES"
                .to_owned();
        for line in parsed {
            let value_type = if line[2].parse::<f64>().is_ok() {
                "real"
            } else {
                "text"
            };
            sql += &format!(
                " ({}, '{}', '{}', '{}', '{}'),",
                id, line[0], line[2], value_type, line[1]
            )
            .to_owned();
        }
        sql.pop();
        sql.push(';');
        let _ = self.api_server.fetch(&sql)?;
        println!("General ship_parameters ok");
        println!("General process end");
        Ok(id as usize)
    }
}
