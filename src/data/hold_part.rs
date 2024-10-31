//! Структура с данными для hold_part
use crate::error::Error;
use crate::Table;


// Структура с данными для hold_part
pub struct HoldPart {
    data: String,
    parsed: Vec<(String, String, String, String, String, String, String, String)>,
}
//
impl HoldPart {
    //
    pub fn new(data: String,) -> Self {
        Self {
            data,
            parsed: Vec::new(),
        }
    }
    //
    pub fn to_string(&self, ship_id: usize) -> String {
        let mut result = format!("DELETE FROM hold_part WHERE ship_id={ship_id};\n\n");
        result += "INSERT INTO hold_part\n  (id, ship_id, space_id, code, group_space_id, group_index, left_bulkhead_code, right_bulkhead_code, bound_x1, bound_x2)\nVALUES\n";
        self.parsed
            .iter()
            .enumerate()
            .for_each(|(id, (space_id, code, group_space_id, group_index, left_bulkhead_code, right_bulkhead_code, bound_x1, bound_x2))| {
                result += &format!("  ({id}, {ship_id}, {space_id}, '{code}', {group_space_id}, {group_index}, ");
                if left_bulkhead_code.len() > 0 {
                    result += &format!(" '{left_bulkhead_code}',");
                } else {
                    result += " NULL,";
                }
                if right_bulkhead_code.len() > 0 {
                    result += &format!(" '{right_bulkhead_code}',");
                } else {
                    result += " NULL,";
                }
                result += &format!(" {bound_x1}, {bound_x2}),\n");
            });
        result.pop();
        result.pop();
        result.push(';');
        result
    } 
}
//
impl Table for HoldPart {
    //
    fn parse(&mut self) -> Result<(), Error> {
     //   dbg!(&self.data);
        println!("HoldPart parse begin");
        let mut data = crate::split_data(&self.data)?;
        data.remove(0);
        self.parsed = data
            .into_iter()
            .filter_map(|line| {
                if line.len() >= 9 {
                    let mut left_bulkhead_code = line[5].to_owned(); // left_hold
                    left_bulkhead_code.retain(|c| c.is_alphanumeric());
                    let mut right_bulkhead_code = line[6].to_owned(); // right_hold
                    right_bulkhead_code.retain(|c| c.is_alphanumeric());
                    Some((
                        line[1].to_owned(), // Space ID
                        line[2].to_owned(), // Code
                        line[3].to_owned(), // Parent Compartment Code
                        line[4].to_owned(), // group_index
                        left_bulkhead_code, 
                        right_bulkhead_code,
                        line[7].to_owned(), // X1 [m]
                        line[8].to_owned(), // X2 [m]
                    ))
                } else {
                    None
                }
            })
            .collect();
     //   dbg!(&self.parsed);
        println!("HoldPart parse ok");
        Ok(())
    }
    //
    fn to_sql(&self, id: usize) -> Vec<String> {
        vec![self.to_string(id)]
    }
    //
    fn to_file(&self, id: usize) {
        std::fs::write("hold_part.sql", self.to_string(id)).expect("Unable to write file hold_part.sql");           
        std::thread::sleep(std::time::Duration::from_secs(1));  
    }
}
