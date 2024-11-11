//! Общие данные по судну
use std::rc::Rc;

use calamine::Data;

use crate::error::Error;
use crate::Table;

/// Общие данные по судну
pub struct ShipGeneral {
    data: Rc<Vec<[Data]>>,
    /// name	value
    parsed: Vec<Vec<String>>,
}
//
impl ShipGeneral {
    //
    pub fn new(data: Rc<Vec<[Data]>>) -> Self {
        Self {
            data,
            parsed: Vec::new(), 
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM vertical_area_strength WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO vertical_area_strength\n  (ship_id, name, value, bound_x1, bound_x2)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|row| {
                result += &format!("  ({ship_id}, '{}', {}, {}, {}),\n", row[0], row[1], row[2], row[3]);
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for ShipGeneral  {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("VerticalAreaStrength parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data.into_iter() {  
            if row.len() != 4 {
                return Err(Error::FromString(format!("VerticalAreaStrength parse error: row.len() != 4, row={:?}", row)));
            }
            self.parsed.push(row);
        }
        println!("VerticalAreaStrength parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize, name: &str) {
        std::fs::write(format!("../{name}/area/vertical_area_strength.sql"), self.to_string(id))
            .expect("Unable to write file vertical_area_strength.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
