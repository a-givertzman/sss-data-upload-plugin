//! Структура с данными для container_slot
use crate::error::Error;
use crate::Table;

/// Структура с данными для container_slot
pub struct Container {
    data: Vec<Vec<String>>,
    /// bay_number, row_number, tier_number
    parsed: Vec<(String, String, String)>,
}
//
impl Container {
    //
    pub fn new(data: Vec<Vec<String>>,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //  
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("UPDATE container_slot SET container_id = NULL WHERE ship_id={ship_id} AND project_id IS NOT DISTINCT FROM NULL;\n\n");
        result += &format!("DELETE FROM container WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO container\n  (project_id, ship_id, iso_code, max_gross_mass, gross_mass, tare_mass)\nVALUES\n  (NULL, 2, '1CC', 36.0, 12.0, 0.0);\n\n";        
        self.parsed.iter().for_each(|(bay_number, row_number, tier_number)| {
            result += "UPDATE\n  container_slot\nSET\n  container_id = 5\nWHERE\n\n";
            result += &format!("  ship_id = {ship_id} AND project_id IS NOT DISTINCT FROM NULL AND bay_number = {bay_number} AND row_number = {row_number} AND tier_number = {tier_number};\n\n");
        });
        result.pop();
        result.pop();
        result.push_str(";\n\n");
        result
    }
}
//
impl Table for Container {
    //
    fn parse(&mut self) -> Result<(), Error> {
        println!("Container parse begin");
        let mut data: Vec<Vec<String>> = self.data.clone().into_iter().filter(|s| s.len() >= 5).collect();
        data.remove(0);
        for row in data.into_iter() {
       //     dbg!(&row);
            self.parsed.push(
                (row[0].clone(),
                row[1].clone(),
                row[2].clone())
            );
        }
        println!("Container parse ok");
        Ok(())
    }
    //
    fn to_file(&self, ship_id: usize, name: &str) {
        std::fs::write(
            format!("../{name}/test/container.sql"),
            self.to_string(ship_id),
        )
        .expect("Unable to write file /test/container.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //
    fn to_sql(&self, ship_id: usize) -> Vec<String> {
        vec![self.to_string(ship_id)]
    }
}
