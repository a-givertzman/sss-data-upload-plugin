//! Структура с данными для bulkhead_place
use crate::error::Error;
use crate::Table;

// Структура с данными для bulkhead_place
pub struct BulkheadPlace {
    data: String,
    parsed: Vec<(String, String, String, String, String, String, String, String, String, String)>,
}
//
impl BulkheadPlace {
    //
    pub fn new(data: String) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //(id, ship_id, space_id, code, name_rus, name_engl, hold_group_id, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z)
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM bulkhead_place WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO bulkhead_place\n  (id, ship_id, space_id, code, name_rus, name_engl, hold_group_id, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z)\nVALUES\n";
        self.parsed
            .iter()
            .enumerate()
            .for_each(|(id, (space_id, code, name_rus, name_engl, hold_group_space_id, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z))| {
                result +=
                    &format!("  ({id}, {ship_id}, {space_id}, '{code}', '{name_rus}', '{name_engl}', (SELECT id FROM hold_group WHERE space_id={hold_group_space_id}), {bound_x1}, {bound_x2}, {mass_shift_x}, {mass_shift_y}, {mass_shift_z}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    }
}
//
impl Table for BulkheadPlace {
    //
    fn parse(&mut self) -> Result<(), Error> {
        //   dbg!(&self.data);
        println!("BulkheadPlace parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() >= 11 {
                    Some((
                        line[1].to_owned(), // Space ID
                        line[2].to_owned(), // Code
                        line[3].to_owned(), // Name  RUS
                        line[4].to_owned(), // Name  ENGL
                        line[5].to_owned(), // Parent Compartment 
                        line[6].to_owned(), // X1 [m]
                        line[7].to_owned(), // X2 [m]
                        line[8].to_owned(), // Xg [m]
                        line[9].to_owned(), // Yg [m]
                        line[10].to_owned(), // Zg [m]
                    ))
                } else {
                    None
                }
            })
            .collect();
        //   dbg!(&self.parsed);
        println!("BulkheadPlace parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("bulkhead_place.sql", self.to_string(id))
            .expect("Unable to write file bulkhead_place.sql");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
