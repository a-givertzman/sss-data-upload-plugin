//! Структура с данными для compartment
use crate::error::Error;
use crate::Table;
use std::rc::Rc;

/// Структура с данными для compartment
pub struct Compartment {
    data: Vec<Vec<String>>,
    /// code  po [t/m3]  M [t]
    parsed: Vec<(String, String, String)>,
}
//
impl Compartment {
    //
    pub fn new(data: Vec<Vec<String>>) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    fn to_string(&self, ship_id: usize) -> String {
        let mut result = String::new();
        self.parsed.iter().for_each(|(space_id, density, mass)| {
            result += &format!("UPDATE compartment SET density={density}, mass={mass} WHERE ship_id={ship_id} AND space_id={space_id};\n");
        });
        result
    }
}
//
impl Table for Compartment {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Compartment parse begin");
        self.parsed = self
            .data
            .iter()
            .filter(|v| {
                v.len() >= 4
                    && match v[3].parse::<f64>() {
                        Ok(mass) => mass > 0.,
                        Err(_) => false,
                    }
            })
            .map(|v| {
                (
                    v[0].clone(),
                    v[2].parse::<f64>()
                        .map_or("NULL".to_owned(), |v| v.to_string()),
                    v[3].clone(),
                )
            })
            .collect();
        println!("Compartment parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/compartment.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/compartment.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
