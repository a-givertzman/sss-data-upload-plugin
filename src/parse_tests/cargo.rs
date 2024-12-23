//! Структура с данными для cargo
use crate::error::Error;
use crate::Table;
use std::fs;

/// Структура с данными для cargo
pub struct Cargo {
    data: Vec<Vec<String>>,
    /// Генеральный груз
    /// name, mass, bound_x1, bound_x2, bound_y1, bound_y2, bound_z1, bound_z2, mass_shift_x, mass_shift_y, mass_shift_z
    parsed: Vec<(String, String, String, String, String, String, String, String, String, String, String)>,
}
//
impl Cargo {
    //
    pub fn new(data: Vec<Vec<String>>,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //  
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM cargo WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO cargo\n  (ship_id, name, mass, timber, is_on_deck, bound_x1, bound_x2, bound_y1, bound_y2, bound_z1, bound_z2,mass_shift_x, mass_shift_y, mass_shift_z, category_id)\nVALUES\n";
        self.parsed.iter().for_each(|(name, mass, mass_shift_x, mass_shift_y, mass_shift_z, bound_x1, bound_x2, bound_y1, bound_y2, bound_z1, bound_z2)| {
            result += &format!("  ({ship_id}, '{name}', {mass}, FALSE, TRUE, {bound_x1}, {bound_x2}, {bound_y1}, {bound_y2}, {bound_z1}, {bound_z2}, {mass_shift_x}, {mass_shift_y}, {mass_shift_z}, 10),\n");
        });
        result.pop();
        result.pop();
        result.push_str(";\n\n");
        result
    }
}
//
impl Table for Cargo {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Cargo parse begin");
        let mut data: Vec<Vec<String>> = self.data.clone().into_iter().filter(|s| s.len() >= 11).collect();
        data.remove(0);
        for row in data.into_iter() {
       //     dbg!(&row);
            self.parsed.push(
                (row[0].clone(),
                row[1].clone(),
                row[2].clone(),
                row[3].clone(),
                row[4].clone(),
                row[5].clone(),
                row[6].clone(),
                row[7].clone(),
                row[8].clone(),
                row[9].clone(),
                row[10].clone())
            );
        }
        println!("Cargo parse ok");
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/cargo.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/cargo.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
