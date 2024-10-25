//! Структура с данными для physical_frame
use std::collections::HashMap;

use crate::error::Error;
use crate::Table;

use super::{Curve, ICurve};

/// Структура с данными для physical_frame
pub struct PhysicalFrame {
    data: Option<String>,
    index_map: HashMap<String, f64>,
    curve: Option<Curve>,
    parsed: Vec<(String, String)>,
}
//
impl PhysicalFrame {
    //
    pub fn new(data: String) -> Self {
        Self {
            data: Some(data),
            index_map: HashMap::new(),
            curve: None,
            parsed: Vec::new(),
        }
    }
    //
    pub fn value(&self, index: &str) -> Result<f64, Error> {
        let index = index.replace(",", ".");
        let converted_index = if index.contains(".") {
            let index_vec: Vec<&str> = index.split(".").collect();
            let first = *self.index_map.get(index_vec[0]).ok_or(Error::FromString(format!("PhysicalFrame value error: no index{} in index_map", index)))?;
            let mut second = ("0.".to_owned() + index_vec[1]).parse::<f64>()?;
            if index.contains("-") {
                second = -second;
            }
            first + second
        } else {
            *self.index_map.get(&index).ok_or(Error::FromString(format!("PhysicalFrame value error: no index{} in index_map", index)))?
        };
     //   dbg!(&index, converted_index);
        if let Some(curve) = self.curve.as_ref() {
            return curve.value(converted_index);
        }
        Err(Error::FromString(format!("PhysicalFrame value error: no value {index}")))
    }
    //
    fn physical_frame(&self, id: usize) -> String {
        let mut result = format!("DELETE FROM physical_frame WHERE ship_id={id};\n\n");
        result += "INSERT INTO physical_frame\n  (ship_id, frame_index, pos_x)\nVALUES\n";
        self.parsed.iter().for_each(|line| {
            result += &format!("  ({}, '{}', {}),\n", id, line.0, line.1);
        });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for PhysicalFrame {
    ///
    fn data(&mut self) -> Option<String> {
        self.data.take()
    }
    ///
    fn parse(&mut self) -> Result<(), Error> {
        println!("PhysicalFrame parse begin");
        let mut data = self.split_data()?;
        data.remove(0);
        let mut curve_data = Vec::new();
        for (index, row) in data.into_iter().enumerate() {
            let key = row.get(0).ok_or(Error::FromString(
                "PhysicalFrame error: no key in row".to_owned(),
            ))?.to_owned();
            let value = row.get(1).ok_or(Error::FromString(
                "PhysicalFrame error: no value in row".to_owned(),
            ))?.parse::<i32>()? as f64 * 0.001;
            self.index_map.insert(key.clone(), index as f64);   
            curve_data.push((index as f64, value));         
            self.parsed.push((key, value.to_string()));

        };
        self.curve = Some(Curve::new_linear(&curve_data));
      //  dbg!(&self.parsed);
        println!("PhysicalFrame parse ok");
        Ok(())
    }
    ///
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.physical_frame(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("physical_frame.sql", self.physical_frame(id)).expect("Unable to write file physical_frame.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
