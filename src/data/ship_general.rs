//! Структура с данными для ship_name и ship_parameters
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::ApiServer;

/// Структура с данными для ship_name и ship_parameters
pub struct General {
    data: String,
    api_server: Rc<RefCell<ApiServer>>,
}
///
impl General {
    ///
    pub fn new(data: String, api_server: Rc<RefCell<ApiServer>>) -> Self {
        Self { data, api_server }
    }
    /// Создание записи в таблице ship_name.
    /// Возвращает id судна
    pub fn process(self) -> Result<usize, Error> {
        println!("General process begin");
        let data: Vec<&str> = self.data.split("\r\n").filter(|s| s.len() > 0).collect();
        let mut map = HashMap::new();
        for line in &data {
            let line = line.split(';').collect::<Vec<&str>>();
            if let (Some(v1), Some(v2)) = (line.first(), line.last()) {
                map.insert(v1.to_owned(), v2.to_owned());
            }
        }
        println!("General parse ok");
        let name = format!(
            "'{}'",
            map.get(&"Name of ship").ok_or(Error::FromString(format!(
                "parse_general no 'Name of ship'!"
            )))?
        );
        let project = map.get(&"Name of project").unwrap_or(&"NULL");
        let year_of_built = map.get(&"Year of built").unwrap_or(&"NULL");
        let place_of_built = map.get(&"Place of built").unwrap_or(&"NULL");
        let imo = map.get(&"IMO number").unwrap_or(&"NULL");
        let mmsi = map.get(&"MMSI").unwrap_or(&"NULL");
        self.api_server.borrow_mut().fetch(&format!(
            "INSERT INTO ship_name AS s(name, project, year_of_built, place_of_built, IMO, MMSI) \
            VALUES ({name}, {project}, {year_of_built}, {place_of_built}, {imo}, {mmsi}) \
            ON CONFLICT ON CONSTRAINT ship_name_unique DO UPDATE \
            SET project={project}, year_of_built={year_of_built}, place_of_built={place_of_built}, IMO={imo}, MMSI={mmsi} \
            WHERE s.name={name};"
        ))?;
        let result = self
            .api_server
            .borrow_mut()
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
        self.api_server
            .borrow_mut()
            .fetch(&format!("DELETE FROM ship_parameters WHERE ship_id={id};").to_owned())?;
        let mut sql =
            "INSERT INTO ship_parameters (ship_id, key, value, value_type, unit) VALUES".to_owned();

        for line in &data {
            let mut line = line.split(';').filter(|line| line.len() >= 2 ).collect::<Vec<&str>>();
            let value = line
            .pop()
            .expect(&format!("General process line error!"));
            let key = line.remove(0);
            let unit = line.pop().unwrap_or("NULL");
            let value_type = if value.parse::<f64>().is_ok() {
                "real"
            } else {
                "text"
            };
            sql += &format!(" ({id}, '{key}', '{value}', '{value_type}', '{unit}'),").to_owned();
        }
        sql.pop();
        sql.push(';');
        let _ = self.api_server.borrow_mut().fetch(&sql)?;
        println!("General ship_parameters ok");
        println!("General process end");
        Ok(id as usize)
    }
}
