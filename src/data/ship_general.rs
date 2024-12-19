//! Структура с данными для ship_name и ship_parameters
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::{ApiServer, Table};

/// Структура с данными для ship_name и ship_parameters
pub struct General {
    data: Vec<Vec<String>>,
    parameters: Vec<(String, f64, String)>,
    map: HashMap<String, String>,
    api_server: Rc<RefCell<ApiServer>>,
}
//
impl General {
    //
    pub fn new(data: Vec<Vec<String>>, api_server: Rc<RefCell<ApiServer>>) -> Self {
        Self {
            data,
            parameters: Vec::new(),
            map: HashMap::new(),
            api_server,
        }
    }
    //
    pub fn ship_name(&self) -> Result<String, Error> {
        Ok(self
            .map
            .get("Ship name")
            .ok_or(Error::FromString(format!(
                "General ship_name error: no ship name in map:{:?}",
                self.map
            )))?
            .clone())
    }
    //
    pub fn ship_id(&self) -> Result<usize, Error> {
        Ok(2)
    }
    //
    fn get(&self, key: &str) -> String {
        if let Some(value) = self.map.get(key) {
            if value.as_str() != "" && value.as_str() != "-" {
                return format!("'{value}'");
            }
        }
        "NULL".to_owned()
    }
    //
    pub fn ship_parameters(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM ship_parameters WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO ship_parameters\n  (ship_id, key, value, unit_id)\nVALUES\n";
        self.parameters
            .iter()
            .for_each(|(name, value, value_type)| {
                let value_type_id = match value_type.as_str() {
                    "-" | "" => "NULL".to_owned(),
                    s => format!("(SELECT id FROM unit WHERE symbol_eng = '{s}')"),
                };
                result += &format!("  ({ship_id}, '{name}', {value}, {value_type_id}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    pub fn ship(&self, ship_id: usize) -> String {
        let name = self.get("Ship name");
        let project = self.get("Project");
        let year_of_built = self.get("Year of build");
        let place_of_built = self.get("Place of build");
        let imo = self.get("IMO number");
        let mmsi = self.get("MMSI");
        let ship_type = self.get("Type of ship");
        let navigation_area = self.get("Navigation area");
        let freeboard_type = self.get("freeboardType");
        let mut result = format!("DELETE FROM ship WHERE id={ship_id};\n\n");
        result += "INSERT INTO ship\n  (id, name, project, year_of_built, place_of_built, IMO, MMSI, ship_type_id, icing_type_id, icing_timber_type_id, navigation_area_id, freeboard_type, geometry_id)\nVALUES\n";
        result += &format!("  ({ship_id}, {name}, {project}, {year_of_built}, {place_of_built}, {imo}, {mmsi}, (SELECT id FROM ship_type WHERE type_rmrs = (SELECT id FROM ship_type_rmrs WHERE title_eng = {ship_type})), 1, 1, (SELECT id FROM navigation_area WHERE area ={navigation_area}), {freeboard_type}, {ship_id});\n\n");
        result
    }
}

impl Table for General {
    //
    fn parse(&mut self) -> Result<(), Error> {
        let data: Vec<Vec<String>> = self.data.clone().into_iter().filter(|s| s.len() >= 3).collect();
        self.parameters = data
            .iter()
            .filter_map(
                |v| match (v[0].clone(), v[2].replace(",", ".").parse::<f64>()) {
                    (name, Ok(value)) => Some((name, value, v[1].to_owned())),
                    _ => None,
                },
            )
            .collect();
        self.map = data
            .iter()
            .map(|v| (v[0].to_owned(), v[2].to_owned()))
            .collect();
        Ok(())
    }
    //   //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.ship(id), self.ship_parameters(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        let mut tmp = String::new();
        tmp += &self.ship(id);
        tmp += &self.ship_parameters(id);
        std::fs::write(format!("../{name}/ship.sql"), tmp).expect("Unable to write file ship.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
