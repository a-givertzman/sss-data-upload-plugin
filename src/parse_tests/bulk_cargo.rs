//! Структура с данными для трюмов
use crate::error::Error;
use crate::Table;

/// Структура с данными для трюмов
pub struct BulkCargo {
    data: Vec<Vec<String>>,
    /// code  po [t/m3]  M [t] cargo_category
    parsed: Vec<(String, String, String, String)>,
}
//
impl BulkCargo {
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
        self.parsed.iter().for_each(|(group_id, density, mass, category_id)| {
            result += &format!("UPDATE hold_compartment SET density={density}, mass={mass}, category_id={category_id} WHERE ship_id={ship_id} AND group_id=(SELECT id FROM hold_group WHERE ship_id={ship_id} AND space_id={group_id});\n");
        });
        result
    }
}
//
impl Table for BulkCargo {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("BulkCargo parse begin");
        self.parsed = self
            .data
            .iter()
            .filter(|v| {
                v.len() >= 5
                    && match v[3].parse::<f64>() {
                        Ok(mass) => mass > 0.,
                        Err(_) => false,
                    }
            })
            .map(|v| {
                (
                    v[0].clone(),
                    v[2].parse::<f64>()
                        .map_or("NULL".to_owned(), |v| (1./v).to_string()),
                    v[3].clone(),
                    match v[4].as_str() {
                        "yes" => 12.to_string(),
                        _ =>  11.to_string(),
                    },
                )
            })
            .collect();
        println!("BulkCargo parse ok");
        //  dbg!(&self.parsed);
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/bulk_cargo.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/bulk_cargo.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
