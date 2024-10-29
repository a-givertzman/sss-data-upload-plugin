//! Структура с данными для load_constant
use std::rc::Rc;

use crate::error::Error;
use crate::Table;

use super::PhysicalFrame;

/// Структура с данными для load_constant
pub struct LoadConstant {
    data: String,
    physical_frame: Rc<PhysicalFrame>,
    parsed: Vec<(String, String, String)>,
}
//
impl LoadConstant {
    //
    pub fn new(data: String, physical_frame: Rc<PhysicalFrame>) -> Self {
        Self {
            data,
            physical_frame,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, id: usize) -> String {
        let mut result = format!("DELETE FROM to_string WHERE ship_id={id};\n\n");
        result +=
            "INSERT INTO to_string\n  (ship_id, mass, bound_x1, bound_x2, category_id)\nVALUES\n";
        self.parsed.iter().for_each(|line| {
            result += &format!("  ({}, {}, {}, {}, 20),\n", id, line.0, line.1, line.2);
        });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for LoadConstant {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("LoadConstant parse begin");
        //    dbg!(&self.data);
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data {
        //   dbg!(&row);
            let fr_min = row.get(0).ok_or(Error::FromString(
                "LoadConstant error: no fr_min in row".to_owned(),
            ))?;
            let fr_max = row.get(1).ok_or(Error::FromString(
                "LoadConstant error: no fr_max in row".to_owned(),
            ))?;
            let mass = row
                .get(2)
                .ok_or(Error::FromString(
                    "LoadConstant error: no mass in row".to_owned(),
                ))?
                .parse::<f64>()?;
            let bound_min_x = self.physical_frame.value(fr_min)?;
            let bound_max_x = self.physical_frame.value(fr_max)?;
            self.parsed.push((
                mass.to_string(),
                bound_min_x.to_string(),
                bound_max_x.to_string(),
            ));
        }
        //  dbg!(&self.parsed);
        println!("LoadConstant parse ok");
        Ok(())
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("load_constant.sql", self.to_string(id))
            .expect("Unable to write file load_constant.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
}
