//! Структура с данными для ship_name и ship_parameters
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::{ApiServer, Table};

/// Структура с данными для ship_name и ship_parameters
pub struct General {
    data: String,
    parsed: HashMap<String, (String, Option<String>)>,
    api_server: Rc<RefCell<ApiServer>>,
}
//
impl General {
    //
    pub fn new (data: String, api_server: Rc<RefCell<ApiServer>>) -> Self {
        Self {
            data,
            parsed: HashMap::new(),
            api_server,
        }
    }
    //
    pub fn ship_id(&self) -> Result<usize, Error> {
        Ok(2)
  /*      println!("General parse ok");
        let name = format!(
            "'{}'",
            self.parsed.get("Name of ship").ok_or(Error::FromString(format!(
                "parse_general no 'Name of ship'!"
            )))?.0
        );
        let mut sql_name = "name".to_owned();
        let mut sql_values = format!("{name}");
        if let Some(project) = self.parsed.get("Name of project") {
            sql_name += &format!(", project");
            sql_values += &format!(", '{}'", project.0);
        }
        if let Some(year_of_built) = self.parsed.get("Year of built") {
            sql_name += &format!(", year_of_built");
            sql_values += &format!(", {}", year_of_built.0.parse::<i32>()?);
        }
        if let Some(place_of_built) = self.parsed.get("Place of built") {
            sql_name += &format!(", place_of_built");
            sql_values += &format!(", '{}'", place_of_built.0);
        }
        if let Some(imo) = self.parsed.get("IMO number") {
            sql_name += &format!(", IMO");
            sql_values += &format!(", {}", imo.0.parse::<i32>()?);
        }
        if let Some(mmsi) = self.parsed.get("MMSI") {
            sql_name += &format!(", MMSI");
            sql_values += &format!(", {}", mmsi.0.parse::<i32>()?);
        }  
        let sql = "INSERT INTO ship_name (".to_owned() + 
        &sql_name + ") VALUES (" + &sql_values + ");";
        self
            .api_server
            .borrow_mut()
            .fetch(&sql)?;        
        let result = self
            .api_server
            .borrow_mut()
            .fetch(&format!(" SELECT id FROM ship_name WHERE name={name}; "))?;
        let id = result[0]
            .get("id")
            .ok_or(Error::FromString(format!(
                "parse_general no ship_id in reply"
            )))?
            .as_u64()
            .ok_or(Error::FromString(format!(
                "parse_general wrong ship_id type in reply"
            )))?;
        Ok(id as usize)*/
    }
    //
    fn split_data(&mut self) -> Result<Vec<Vec<String>>, Error> {
        Ok(self
            .data
            .replace(" ", "")
            .replace(",", ".")
            .split("\r\n")
            .map(|line| {
                line.split(';')
                    .filter(|s| s.len() > 0)
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .filter(|s| s.len() > 0)
            .collect())
    }
}

impl Table for General {
    //
    fn parse(&mut self) -> Result<(), Error> {
        let data: Vec<Vec<&str>> = 
            self.data
            .split("\r\n")
            .filter(|s| s.len() > 0)
            .map(|s| s.split(';').collect())
            .collect();
        for line in &data {
            match line.len() {
                2 => self.parsed.insert(line[0].to_owned(), (line[1].to_owned(), None)),
                3 => self.parsed.insert(line[0].to_owned(), (line[2].to_owned(), Some(line[1].to_owned()))),
                _ => return Err(Error::FromString(
                    "General parse error: no data! line:{line}".to_owned(),
                )),                
            };
        }
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("DELETE FROM ship_parameters WHERE ship_id={id};"));
        let mut sql1 =
            "INSERT INTO ship_parameters (ship_id, key, value, value_type) VALUES".to_owned();
        let mut sql2 =
            "INSERT INTO ship_parameters (ship_id, key, value, value_type, unit) VALUES".to_owned();
        for (key, (value, unit)) in &self.parsed {
            let value_type = if value.parse::<f64>().is_ok() {
                "real"
            } else {
                "text"
            };
            if let Some(unit) = unit {
                sql2 += &format!(" ({id}, '{key}', '{value}', '{value_type}', '{unit}'),").to_owned();
            } else {
                sql1 += &format!(" ({id}, '{key}', '{value}', '{value_type}'),").to_owned();
            }
        }
        sql1.pop();
        sql1.push(';');
        sql2.pop();
        sql2.push(';');
        result.push(sql1);
        result.push(sql2);
        result
    }
    
    fn to_file(&self, id: usize) {
        //TODO
    }
    
    fn data_to_sql(&self, data: &Vec<(f64, f64)>, table_name: &str, ship_id: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(std::format!(" DELETE FROM {table_name} WHERE ship_id={ship_id};\n\n"));
        let mut sql = std::format!(" INSERT INTO {table_name}\n  (ship_id, key, value)\nVALUES\n");
        data.iter().for_each(|(k, v)| {
            sql += &std::format!("  ({ship_id}, {k}, {v}),\n");
        });
        sql.pop();
        sql.pop();
        sql.push(';');
    //    dbg!(&sql);
        result.push(sql);
        result
    }
}
