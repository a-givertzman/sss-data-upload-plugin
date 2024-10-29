//! Структура с данными для bonjean_frame
use crate::error::Error;
use crate::Table;

/// Структура с данными для bonjean_frame
pub struct Windage {
    data: String,
    /// d [m], Z cl sub [m], Lwl [m], Bwl [m], Av CS [m2], X  Av CS [m], Mvx CS [m3], Z Av CS [m], Mvz CS  [m3], A bow 0,15L [m2]
    parsed: Vec<Vec<f64>>,
}
///
impl  Windage {
    ///
    pub fn new(data: String) -> Self {
        Self {
            data,
            parsed: Vec::new(), 
        }
    }
    //
    pub fn to_string(&self, table_name: &str, ship_id: usize, value_index: usize) -> String {
        let mut result = format!("DELETE FROM {table_name} WHERE ship_id={ship_id};\n\n");
        result += &format!("INSERT INTO {table_name}\n  (ship_id, key, value)\nVALUES\n");
        self.parsed
            .iter()
            .for_each(|row| {
                result += &format!("  ({ship_id}, {}, {}),\n", row[0], row[value_index]);
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
    //
    pub fn waterline_length(&self, ship_id: usize) -> String {
        self.to_string("waterline_length", ship_id, 2)
    }
    //
    pub fn volume_shift(&self, ship_id: usize) -> String {
        self.to_string("volume_shift", ship_id, 1)
    }
    //
    pub fn bow_area(&self, ship_id: usize) -> String {
        self.to_string("bow_area", ship_id, 9)
    }
    //
    pub fn vertical_area_stability(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM vertical_area_stability WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO vertical_area_stability\n  (ship_id, draught, area, moment_x, moment_z)\nVALUES\n";
        self.parsed
            .iter()
            .for_each(|row| {
                result += &format!("  ({ship_id}, {}, {}, {}, {}),\n", row[0], row[4], row[6], row[8]);
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for Windage  {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Windage parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        for row in data.into_iter() {  
            if row.len() != 10 {
                return Err(Error::FromString(format!("Windage parse error: row.len() != 10, row={:?}", row)));
            }
            let mut values = Vec::new();          
            for value in row {
                values.push(value.parse::<f64>()?);
            }
            self.parsed.push(values);
        }
        println!("Windage parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.waterline_length(id), self.volume_shift(id), self.bow_area(id), self.vertical_area_stability(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("waterline_length.sql", self.waterline_length(id))
            .expect("Unable to write file waterline_length.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::fs::write("volume_shift.sql", self.volume_shift(id))
            .expect("Unable to write file volume_shift.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::fs::write("bow_area.sql", self.bow_area(id))
            .expect("Unable to write file bow_area.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::fs::write("vertical_area_stability.sql", self.vertical_area_stability(id))
            .expect("Unable to write file vertical_area_stability.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
